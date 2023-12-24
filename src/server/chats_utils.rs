use crate::server::db::POOL;
use futures::future::join_all;
use sqlx::{prelude::FromRow, types::Uuid};
use std::sync::Arc;
use teloxide::{requests::Requester, types::Message, Bot, RequestError};

use super::binance_api::Statistics;

#[derive(Debug, FromRow)]
#[allow(dead_code)]
pub struct Chat {
    pub chat_id: Uuid,
    telegram_chat_id: String,
}

pub async fn get_chat(telegram_chat_id: String) -> Result<Option<Chat>, sqlx::Error> {
    let pool: &Arc<sqlx::Pool<sqlx::Postgres>> = POOL.get().expect("Pool has not been initialized");
    // let q = "SELECT * FROM chats WHERE telegram_chat_id = $1";
    let chat: Option<Chat> = sqlx::query_as!(
        Chat,
        "SELECT * FROM chats WHERE telegram_chat_id = $1",
        telegram_chat_id
    )
    .fetch_optional(&**pool)
    .await?;
    // let chat: Option<PgRow> = sqlx::query(q)
    //     .bind(telegram_chat_id)
    //     .fetch_optional(&**pool)
    //     .await?;

    // match chat {
    //     Some(row) => Ok(Some(Chat {
    //         chat_id: row.get("chat_id"),
    //         telegram_chat_id: row.get("telegram_chat_id"),
    //     })),
    //     None => Ok(None),
    // }

    Ok(chat)
}

pub async fn subscribe(telegram_chat_id: String) -> Result<(), sqlx::Error> {
    let pool: &Arc<sqlx::Pool<sqlx::Postgres>> = POOL.get().expect("Pool has not been initialized");
    let q = "INSERT INTO chats (telegram_chat_id) VALUES ($1)";
    sqlx::query(q)
        .bind(&telegram_chat_id)
        .execute(&**pool)
        .await?;

    Ok(())
}

pub async fn unsubscribe(chat_id: Uuid) -> Result<(), sqlx::Error> {
    let pool: &Arc<sqlx::Pool<sqlx::Postgres>> = POOL.get().expect("Pool has not been initialized");
    let q = "DELETE FROM chats WHERE chat_id = $1";
    sqlx::query(q).bind(&chat_id).execute(&**pool).await?;

    Ok(())
}

pub async fn send_daily_stats_all(bot: Arc<Bot>) -> Result<(), sqlx::Error> {
    let pool: &Arc<sqlx::Pool<sqlx::Postgres>> = POOL.get().expect("Pool has not been initialized");
    let q: &str = "SELECT telegram_chat_id FROM chats";

    let telegram_chat_ids: Vec<String> = sqlx::query_as::<_, (String,)>(q)
        .fetch_all(&**pool)
        .await?
        .into_iter()
        .map(|row| row.0)
        .collect();

    let stats: Statistics = Statistics::get_stats().await.unwrap();
    let mut tasks: Vec<tokio::task::JoinHandle<()>> = Vec::new();

    let stats_text = match stats.has_none() {
        false => {
            format!(
                r#"BTCUSDT statistics:

Price - {} USDT"#,
                stats.current_price.unwrap()
            )
        }
        true => {
            format!(r#"Something is wrong with getting statistics right now"#)
        }
    };

    for chat_id in telegram_chat_ids {
        let bot_clone = bot.clone();
        let stats_text_clone = stats_text.clone();

        let task = tokio::spawn(async move {
            match send_daily_stats_single(chat_id.clone(), &bot_clone, stats_text_clone).await {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error while sending daily stats: {}", e);
                    let _ = &bot_clone.send_message(chat_id, "Something went wrong with sending you daily statistics, try to get them with running /getstats").await;
                }
            };
        });

        tasks.push(task);
    }

    let _results: Vec<Result<(), tokio::task::JoinError>> = join_all(tasks).await;

    Ok(())
}

pub async fn send_daily_stats_single(
    telegram_chat_id: String,
    bot: &Arc<Bot>,
    stats_message: String,
) -> Result<Message, RequestError> {
    bot.send_message(telegram_chat_id, stats_message).await
}

pub async fn send_help(bot: Bot, msg: Message) -> Result<Message, RequestError> {
    bot.send_message(
        msg.chat.id,
        r#"These commands are available:

/help - Get this message
/subscribe - Subscribe to the daily Bitcoin statistics
/unsubscribe - Unsubscribe from the daily Bitcoin statistics
/getstats - Get Bitoin statistics now with no need to be subscribed"#,
    )
    .await
}

use crate::server::db::POOL;
use std::sync::Arc;
use teloxide::{requests::Requester, types::Message, Bot, RequestError};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Chat {
    chat_id: String,
    telegram_chat_id: String,
}

pub async fn get_chat(telegram_chat_id: String) -> Result<Option<Chat>, sqlx::Error> {
    let pool: &Arc<sqlx::Pool<sqlx::Postgres>> = POOL.get().expect("Pool has not been initialized");
    let chat: Option<Chat> = sqlx::query_as!(
        Chat,
        "SELECT * FROM chats WHERE telegram_chat_id = $1",
        telegram_chat_id
    )
    .fetch_optional(&**pool)
    .await?;

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

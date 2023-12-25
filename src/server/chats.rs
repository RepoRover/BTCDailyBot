use super::{
    binance_api::Statistics,
    chats_utils::{
        get_chat, send_daily_stats_all, send_daily_stats_single, subscribe, unsubscribe,
    },
    side_utils::print_error,
};
use std::{env, sync::Arc};
use teloxide::{requests::Requester, types::Message, Bot, RequestError};

pub async fn handle_subscription(bot: Bot, msg: Message) -> Result<Message, RequestError> {
    match get_chat(msg.chat.id.to_string()).await.unwrap() {
        Some(_) => {
            bot.send_message(msg.chat.id, "You are already subscribed")
                .await
        }
        None => match subscribe(msg.chat.id.to_string()).await {
            Ok(_) => {
                bot.send_message(msg.chat.id, "You are now subscribed")
                    .await
            }
            Err(e) => {
                print_error("Error subscribing", e);
                bot.send_message(msg.chat.id, "Failed to subscribe, try again later")
                    .await
            }
        },
    }
}

pub async fn handle_unsubscribtion(bot: Bot, msg: Message) -> Result<Message, RequestError> {
    match get_chat(msg.chat.id.to_string()).await.unwrap() {
        Some(chat_data) => match unsubscribe(chat_data.chat_id).await {
            Ok(_) => {
                bot.send_message(msg.chat.id, "You are no longer subscribed")
                    .await
            }
            Err(e) => {
                print_error("Error unsubscribing", e);
                bot.send_message(msg.chat.id, "Failed to unsubscribe, try again later")
                    .await
            }
        },
        None => {
            bot.send_message(msg.chat.id, "You are not yet subscribed")
                .await
        }
    }
}

pub async fn handle_daily(bot: Bot) -> Result<Message, RequestError> {
    let admin_chat_id: String = env::var("ADMIN_TCID").expect("No admin chat id provided");
    let bot_arc: Arc<Bot> = Arc::new(bot);
    let bot_clone: Arc<Bot> = bot_arc.clone();

    match send_daily_stats_all(bot_arc).await {
        Ok(_) => {
            bot_clone
                .send_message(admin_chat_id, "Daily stats are sent, well done!")
                .await
        }
        Err(e) => {
            print_error("Failed to send stats", e);
            bot_clone
                .send_message(admin_chat_id, "Daily stats are sent")
                .await
        }
    }
}

pub async fn handle_stats_now(bot: Bot, msg: Message) -> Result<Message, RequestError> {
    let arc_bot: Arc<Bot> = Arc::new(bot);
    let stats: Statistics = Statistics::get_stats().await.unwrap();

    let stats_text = match stats.has_none() {
        false => {
            format!(
                r#"BTCUSDT statistics:
            
Price - {} USDT"#,
                stats.current_price.unwrap()
            )
        }
        true => {
            print_error(
                "Send statistics now",
                "Something is wrong with getting statistics, None fields found",
            );
            format!(r#"Something is wrong with getting statistics right now"#)
        }
    };

    send_daily_stats_single(msg.chat.id.to_string(), &arc_bot, stats_text).await
}

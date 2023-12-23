use super::chats_utils::{get_chat, subscribe};
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
                eprintln!("Error subscribing: {}", e);
                bot.send_message(msg.chat.id, "Failed to subscribe").await
            }
        },
    }
}

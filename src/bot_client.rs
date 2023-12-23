use crate::server::{chats::handle_subscription, chats_utils::send_help};
use chrono::{Datelike, Timelike, Utc};
use std::sync::Arc;
use teloxide::{prelude::*, RequestError};
use tokio::{
    sync::{broadcast, Mutex},
    time,
};

#[derive(Debug)]
enum Commands {
    Help,
    Subscribe,
    Unsubscribe,
    GetStats,
}

fn parse_command(input: &str) -> Option<Commands> {
    match input.to_lowercase().as_str() {
        "/help" => Some(Commands::Help),
        "/subscribe" => Some(Commands::Subscribe),
        "/unsubscribe" => Some(Commands::Unsubscribe),
        "/getstats" => Some(Commands::GetStats),
        _ => None,
    }
}

pub struct ClientBot {
    bot: Bot,
}
impl ClientBot {
    pub fn new() -> Self {
        Self {
            bot: Bot::from_env(),
        }
    }

    pub async fn handle_msgs(&self) {
        let bot_clone: Bot = self.bot.clone();

        teloxide::repl(bot_clone, |bot: Bot, msg: Message| async move {
            match msg.text() {
                Some(text) => match parse_command(text) {
                    Some(command) => match command {
                        Commands::Help => send_help(bot, msg).await?,
                        Commands::Subscribe => handle_subscription(bot, msg).await?,
                        Commands::Unsubscribe => {
                            bot.send_message(msg.chat.id, "Unsubscribe command received")
                                .await?
                        }
                        Commands::GetStats => {
                            bot.send_message(msg.chat.id, "You are getting Bitcoin statistics now")
                                .await?
                        }
                    },
                    None => send_help(bot, msg).await?,
                },
                None => send_help(bot, msg).await?,
            };
            Ok(())
        })
        .await;
    }

    pub async fn regular_check(
        &self,
        mut shutdown_signal: broadcast::Receiver<()>,
        day: Arc<Mutex<u32>>,
    ) -> Result<(), RequestError> {
        let mut interval: time::Interval = time::interval(time::Duration::from_secs(60));
        loop {
            tokio::select! {
                _ = interval.tick() => {
                    Self::daily_stats(self, &day).await?;
                }
                _ = shutdown_signal.recv() => {
                    eprintln!("Shutting down periodic task");
                    return Ok(())
                }
            }
        }
    }

    async fn daily_stats(&self, day: &Arc<Mutex<u32>>) -> Result<(), RequestError> {
        let now: chrono::prelude::DateTime<Utc> = Utc::now();
        let mut day: tokio::sync::MutexGuard<'_, u32> = day.lock().await;

        if *day != now.day() && now.hour() == 7 {
            println!("Giving new daily stats");
            // Send daily stats to everyone who subscribed here
            // self.bot
            //     .send_message("".to_string(), "daily stats")
            //     .await?;
            *day = now.minute();
        }
        Ok(())
    }
}

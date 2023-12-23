mod bot_client;
use bot_client::ClientBot;
use chrono::{Datelike, Utc};
use std::sync::Arc;
use tokio::{
    signal,
    sync::{broadcast, Mutex},
};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let (shutdown_tx, shutdown_rx) = broadcast::channel(1);
    let day: Arc<Mutex<u32>> = Arc::new(Mutex::new(Utc::now().day()));

    let bot_handle: tokio::task::JoinHandle<()> = tokio::spawn({
        let bot: ClientBot = ClientBot::new();
        let shutdown_tx: broadcast::Sender<()> = shutdown_tx.clone();
        async move {
            bot.handle_msgs().await;
            let _ = shutdown_tx.send(());
        }
    });

    let daily_handle: tokio::task::JoinHandle<()> = tokio::spawn({
        let cloned_day: Arc<Mutex<u32>> = day.clone();
        let bot: ClientBot = ClientBot::new();
        async move {
            let _ = bot
                .regular_check(shutdown_rx.resubscribe(), cloned_day)
                .await;
        }
    });

    tokio::select! {
        _ = signal::ctrl_c() => {
            println!("Ctrl-C received, shutting down");
        }
        _ = bot_handle => {
            println!("Bot stopped");
        }
    }

    let _ = shutdown_tx.send(());
    let _ = daily_handle.await;
}

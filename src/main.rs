mod bot_client;
mod server;

use bot_client::ClientBot;
use chrono::{Datelike, Utc};
use server::db::init_pool;
use server::db::POOL;
use server::side_utils::print_error;
use std::sync::Arc;
use tokio::{
    signal,
    sync::{broadcast, Mutex},
};

#[tokio::main]
async fn main() {
    print_error("Starting application", "");
    dotenv::dotenv().ok();
    print_error("Evironment is set", "");

    let pool: Arc<sqlx::Pool<sqlx::Postgres>> =
        init_pool().await.expect("Failed to initialize pool");
    POOL.set(pool).expect("Failed to set global pool");

    let (shutdown_tx, shutdown_rx) = broadcast::channel(1);

    let day: Arc<Mutex<u32>> = Arc::new(Mutex::new(Utc::now().day()));
    let bot: Arc<ClientBot> = Arc::new(ClientBot::new());

    let bot_clone: Arc<ClientBot> = bot.clone();
    let messages_handle: tokio::task::JoinHandle<()> = tokio::spawn({
        let shutdown_tx: broadcast::Sender<()> = shutdown_tx.clone();
        async move {
            bot_clone.handle_msgs().await;
            let _ = shutdown_tx.send(());
        }
    });

    let daily_handle: tokio::task::JoinHandle<()> = tokio::spawn({
        let cloned_day: Arc<Mutex<u32>> = day.clone();
        async move {
            let _ = bot
                .regular_check(shutdown_rx.resubscribe(), cloned_day)
                .await;
        }
    });

    tokio::select! {
        _ = signal::ctrl_c() => {
            print_error("Shut down", "Ctrl-C received, shutting down");
        }
        _ = messages_handle => {
            print_error("Bot error", "Bot stopped");
        }
    }

    let _ = shutdown_tx.send(());
    let _ = daily_handle.await;
}

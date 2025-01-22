use teloxide::prelude::*;
use witch_blvde_sapce_rbot::{apply_command_handler};

#[tokio::main]
async fn main() {
    let bot = Bot::from_env();
    apply_command_handler(bot.clone()).await;
}

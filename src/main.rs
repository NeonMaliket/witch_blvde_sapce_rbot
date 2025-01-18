use teloxide::prelude::*;
use witch_blvde_sapce_rbot::BotMetadata;
use witch_blvde_sapce_rbot::message_handler::attach_message_handlers;

#[tokio::main]
async fn main() {
    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        let metadata = BotMetadata::new(&bot, &msg);
        attach_message_handlers(&metadata).await;
        let chat_id = msg.chat.id;
        bot.send_dice(chat_id).await?;
        Ok(())
    })
    .await;
}

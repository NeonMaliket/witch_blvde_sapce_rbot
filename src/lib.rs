use crate::command_handler::{answer, CustomBotCommand};
use crate::message_buttons_handler::message_button_callback;
use crate::text_handler::common_text_handler;
use teloxide::prelude::*;

mod command_handler;
mod domain;
mod keyboards;
mod message_buttons_handler;
mod messages;
mod text_handler;

pub async fn apply_command_handler(bot: Bot) {
    println!("Command handler called!");

    let command_handler = Update::filter_message()
        .filter_command::<CustomBotCommand>()
        .endpoint(answer);

    let common_message_handler = Update::filter_message()
        .endpoint(|bot: Bot, msg: Message| async move { common_text_handler(bot, msg).await });
    let message_button_callback_handler = Update::filter_callback_query().endpoint(message_button_callback);
    let handler = dptree::entry()
        .branch(command_handler)
        .branch(common_message_handler)
        .branch(message_button_callback_handler);

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

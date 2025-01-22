use crate::command_handler::{answer, CustomBotCommand};
use crate::message_buttons_handler::message_button_callback;
use teloxide::prelude::*;

mod command_handler;
mod domain;
mod keyboards;
mod message_buttons_handler;
mod messages;

pub async fn apply_command_handler(bot: Bot) {
    println!("Command handler called!");

    Dispatcher::builder(
        bot,
        dptree::entry()
            .branch(
                Update::filter_message()
                    .filter_command::<CustomBotCommand>()
                    .endpoint(answer),
            )
            .branch(Update::filter_callback_query().endpoint(message_button_callback)),
    )
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
}

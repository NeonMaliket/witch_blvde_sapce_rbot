use crate::domain::repository::HeroBuildRepository;
use crate::keyboards::hero_build_keyboard;
use crate::messages::{BuildNotFoundMessageResponse, MessageResponse};
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::{Message, Requester, ResponseResult};
use teloxide::types::ParseMode;
use teloxide::utils::command::BotCommands;
use teloxide::Bot;
use teloxide_macros::BotCommands;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "*Witch Blvde Space:*")]
pub enum CustomBotCommand {
    #[command(description = "Space commands")]
    Help,
    #[command(description = "Current active builds")]
    Builds,
}

pub async fn answer(bot: Bot, msg: Message, cmd: CustomBotCommand) -> ResponseResult<()> {
    let chat_id = msg.chat.id;
    match cmd {
        CustomBotCommand::Help => {
            bot.send_message(chat_id, CustomBotCommand::descriptions().to_string())
                .parse_mode(ParseMode::MarkdownV2)
                .await?
        }
        CustomBotCommand::Builds => {
            let index: u32 = 1;
            let message = find_first_build_message().text();
            bot.send_message(chat_id, message)
                .parse_mode(ParseMode::MarkdownV2)
                .reply_markup(hero_build_keyboard(index))
                .await?
        }
    };

    Ok(())
}

fn find_first_build_message() -> Box<dyn MessageResponse> {
    let repository = HeroBuildRepository::new();
    let first_build = repository.find_first_build();
    match first_build {
        None => Box::new(BuildNotFoundMessageResponse),
        Some(build) => Box::new(build)
    }
}
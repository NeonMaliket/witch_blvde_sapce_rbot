use crate::domain::entity::HeroBuild;
use crate::domain::repository::HeroBuildRepository;
use crate::keyboards::hero_build_keyboard;
use crate::messages::{BuildNotFoundMessageResponse, MessageResponse};
use teloxide::payloads::{SendMessageSetters, SendPhotoSetters};
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
    let username = msg
        .from
        .unwrap()
        .username
        .unwrap_or_else(|| "".to_string())
        .clone();
    let repository = HeroBuildRepository::new();

    match cmd {
        CustomBotCommand::Help => {
            bot.send_message(chat_id, CustomBotCommand::descriptions().to_string())
                .parse_mode(ParseMode::MarkdownV2)
                .await?
        }
        CustomBotCommand::Builds => {
            let index: usize = 1;
            let first_build = repository
                .find_first_build()
                .await
                .unwrap_or(HeroBuild::default());
            let message = if first_build.id.is_some() {
                first_build.text()
            } else {
                BuildNotFoundMessageResponse {}.text()
            };
            let input_file = first_build.input_file();

            if input_file.is_some() {
                bot.send_photo(chat_id, input_file.unwrap())
                    .caption(message)
                    .parse_mode(ParseMode::MarkdownV2)
                    .reply_markup(hero_build_keyboard(index, username.as_str()))
                    .await?
            } else {
                bot.send_message(chat_id, message)
                    .parse_mode(ParseMode::MarkdownV2)
                    .reply_markup(hero_build_keyboard(index, username.as_str()))
                    .await?
            }
        }
    };

    Ok(())
}

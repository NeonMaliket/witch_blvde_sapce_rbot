use crate::domain::repository::HeroBuildRepository;
use crate::keyboards::hero_build_keyboard;
use crate::messages::{BuildNotFoundMessageResponse, MessageResponse};
use button_callback::*;
use teloxide::payloads::EditMessageTextSetters;
use teloxide::prelude::{CallbackQuery, Requester, ResponseResult};
use teloxide::types::{ChatId, Message, MessageId};
use teloxide::{Bot, RequestError};

pub mod message_type {
    pub const HERO_BUILDS: &str = "HERO_BUILDS";
}

pub mod button_callback {
    pub(crate) const NEXT_BUTTON: &str = "NEXT_BUTTON";
    pub(crate) const PREVIOUS_BUTTON: &str = "PREVIOUS_BUTTON";
    pub(crate) const SHARE_BUILD_BUTTON: &str = "ADD_BUILD_BUTTON";
    pub(crate) const ADMIN_BUILD_BUTTON: &str = "ADMIN_BUILD_BUTTON";
}

pub async fn message_button_callback(
    bot: Bot,
    callback_query: CallbackQuery,
) -> ResponseResult<()> {
    if let Some(data) = &callback_query.data {
        let message = callback_query.message.unwrap();
        let chat_id = message.chat().id;
        let message_id = message.id();

        if data.contains(&message_type::HERO_BUILDS) {
            hero_build_callback(data, &chat_id, &bot, &message_id).await;
        }
    }

    ResponseResult::Ok(())
}

async fn hero_build_callback(data: &str, chat_id: &ChatId, bot: &Bot, message_id: &MessageId) {
    let callback_data = CallbackData::from(data.to_string());
    println!("Message type: {}", callback_data.message_type);

    let result = match callback_data.button_type.as_str() {
        NEXT_BUTTON => {
            let incremented: u32 = callback_data.incremented_index();
            let message = hero_build_message_response(incremented).text();
            bot.edit_message_text(*chat_id, *message_id, message)
                .reply_markup(hero_build_keyboard(incremented))
                .await
        }
        PREVIOUS_BUTTON => match callback_data.decremented_index() {
            Some(decrement) => {
                let message = hero_build_message_response(decrement).text();
                bot.edit_message_text(*chat_id, *message_id, message)
                    .reply_markup(hero_build_keyboard(decrement))
                    .await
            }
            None => Result::Err(RequestError::MigrateToChatId(*chat_id)),
        },
        ADMIN_BUILD_BUTTON => {
            bot.send_message(*chat_id, "Pls send screenshot of new build".to_string())
                .await
        }
        SHARE_BUILD_BUTTON => {
            bot.send_message(
                *chat_id,
                format!(
                    "Pls send screenshots of you build for index: {}",
                    callback_data.index
                ),
            )
            .await
        }
        _ => bot.send_message(*chat_id, "Unknown button clicked").await,
    };
    wrap_error(result);
}

fn wrap_error(result: Result<Message, RequestError>) {
    if let Err(e) = result {
        println!("[ERROR]: [{}]", e);
    }
}

fn hero_build_message_response(index: u32) -> Box<dyn MessageResponse> {
    let repo = HeroBuildRepository::new();
    let hero_build = repo.find_build_by_index(index);

    match hero_build {
        None => Box::new(BuildNotFoundMessageResponse),
        Some(build) => Box::new(build),
    }
}

struct CallbackData {
    button_type: String,
    message_type: String,
    index: u32,
}

impl CallbackData {
    fn from(message: String) -> CallbackData {
        println!("Trying to split message: {}", message);
        let (button, index) = message.split_once('-').unwrap();
        let (button, message_type) = button.split_once(':').unwrap();
        let index: u32 = index
            .parse()
            .expect("Failed to parse index in CallbackData");

        CallbackData {
            button_type: button.to_string(),
            message_type: message_type.to_string(),
            index,
        }
    }

    fn incremented_index(&self) -> u32 {
        self.index + 1
    }

    fn decremented_index(&self) -> Option<u32> {
        if self.index > 1 {
            Some(self.index - 1)
        } else {
            None
        }
    }
}

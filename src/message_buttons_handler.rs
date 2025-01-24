use crate::domain::repository::{HeroBuildRepository, STORAGE};
use crate::keyboards::{hero_build_keyboard, new_build_keyboard};
use crate::message_buttons_handler::button_callback::hero_builds::*;
use crate::message_buttons_handler::button_callback::new_build::{ADD_DESC, ADD_PHOTO, ADD_TITLE, SAVE_BUILD};
use crate::messages::{BuildNotFoundMessageResponse, MessageResponse};
use std::ops::Deref;
use teloxide::payloads::{EditMessageTextSetters, SendMessageSetters};
use teloxide::prelude::{CallbackQuery, Requester, ResponseResult};
use teloxide::types::{ChatId, Message, MessageId, ParseMode};
use teloxide::{Bot, RequestError};

pub mod message_type {
    pub const HERO_BUILDS: &str = "HERO_BUILDS";
    pub const NEW_BUILD: &str = "NEW_BUILD";
}

pub mod button_callback {
    pub mod hero_builds {
        pub(crate) const NEXT_BUTTON: &str = "NEXT_BUTTON";
        pub(crate) const PREVIOUS_BUTTON: &str = "PREVIOUS_BUTTON";
        pub(crate) const SHARE_BUILD_BUTTON: &str = "ADD_BUILD_BUTTON";
        pub(crate) const ADMIN_BUILD_BUTTON: &str = "ADMIN_BUILD_BUTTON";
    }

    pub mod new_build {
        pub(crate) const NEW_BUILD_BUTTON: &str = "ADMIN_BUILD_BUTTON";

        pub(crate) const ADD_PHOTO: &str = "ADD_PHOTO";
        pub(crate) const ADD_TITLE: &str = "ADD_TITLE";
        pub(crate) const ADD_DESC: &str = "ADD_DESC";
        pub(crate) const SAVE_BUILD: &str = "SAVE_BUILD";
    }
}

pub async fn message_button_callback<'a>(
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
        if data.contains(&message_type::NEW_BUILD) {
            STORAGE.lock().await.default_build_for(chat_id.clone());
            new_build_callback(data, &chat_id, &bot, &message_id).await;
        }
    }

    ResponseResult::Ok(())
}

async fn new_build_callback(
    data: &str,
    chat_id: &ChatId,
    bot: &Bot,
    message_id: &MessageId,
) {
    let callback_data = CallbackData::from(data.to_string());
    let result = match callback_data.button_type.as_str() {
        ADD_PHOTO => {
            STORAGE
                .lock()
                .await
                .update_last_action(chat_id.clone(), ADD_PHOTO);
            bot.edit_message_text(*chat_id, *message_id, "NEW BUILD ADD PHOTO")
                .reply_markup(new_build_keyboard())
                .await
        }
        ADD_TITLE => {
            STORAGE
                .lock()
                .await
                .update_last_action(chat_id.clone(), ADD_TITLE);
            bot.edit_message_text(*chat_id, *message_id, "NEW BUILD ADD TITLE")
                .reply_markup(new_build_keyboard())
                .await
        }
        ADD_DESC => {
            STORAGE
                .lock()
                .await
                .update_last_action(chat_id.clone(), ADD_DESC);
            bot.edit_message_text(*chat_id, *message_id, "NEW BUILD ADD DESC")
                .reply_markup(new_build_keyboard())
                .await
        }
        SAVE_BUILD => {
            let response_text = STORAGE
                .lock()
                .await
                .get_build(chat_id).unwrap()
                .text();
            println!("SAVE BUILD");
            bot.edit_message_text(*chat_id, *message_id, response_text)
                .reply_markup(new_build_keyboard())
                .await
        }
        _ => bot.send_message(*chat_id, "Unknown button clicked").await,
    };
    wrap_error(result);
    println!("Message type: {}", callback_data.message_type);
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
                .parse_mode(ParseMode::MarkdownV2)
                .reply_markup(new_build_keyboard())
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

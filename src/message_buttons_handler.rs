use crate::domain::entity::HeroBuild;
use crate::domain::repository::{current_build, is_admin, remove_last_action, update_last_action, HeroBuildRepository, STORAGE};
use crate::keyboards::{hero_build_keyboard, new_build_keyboard};
use crate::message_buttons_handler::button_callback::hero_builds::*;
use crate::message_buttons_handler::button_callback::new_build::{ADD_DESC, ADD_PHOTO, ADD_TITLE, SAVE_BUILD};
use crate::messages::MessageResponse;
use teloxide::payloads::EditMessageMediaSetters;
use teloxide::payloads::{EditMessageCaptionSetters, SendMessageSetters};
use teloxide::prelude::{CallbackQuery, Requester, ResponseResult};
use teloxide::types::{ChatId, InputMedia, InputMediaPhoto, Message, MessageId, ParseMode};
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
        let username = callback_query
            .from
            .username
            .unwrap_or_else(|| "".to_string());
        let chat_id = message.chat().id;
        let message_id = message.id();
        if data.contains(&message_type::HERO_BUILDS) {

            hero_build_callback(data, &chat_id, &bot, &message_id, username.as_str()).await;
        }
        if data.contains(&message_type::NEW_BUILD) && is_admin(username.as_str()) {
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
            send_new_build_message(
                chat_id,
                bot,
                message_id,
                "Please share a screenshot of your new build",
            )
            .await
        }
        ADD_TITLE => {
            STORAGE
                .lock()
                .await
                .update_last_action(chat_id.clone(), ADD_TITLE);
            send_new_build_message(
                chat_id,
                bot,
                message_id,
                "Please add title for you new build",
            )
            .await
        }
        ADD_DESC => {
            STORAGE
                .lock()
                .await
                .update_last_action(chat_id.clone(), ADD_DESC);
            send_new_build_message(
                chat_id,
                bot,
                message_id,
                "Please add description for your new build",
            )
            .await
        }
        SAVE_BUILD => {
            let build = current_build(chat_id).await;
            let res = HeroBuildRepository::new().save(build.clone()).await;
            match res {
                Ok(_) => {
                    println!("Successfully saved build.");
                }
                Err(_) => {
                    println!("Saving error");
                }
            }
            let response_text = build.text();
            println!("SAVE BUILD");
            remove_last_action(chat_id).await;
            bot.edit_message_caption(*chat_id, *message_id)
                .caption(response_text)
                .parse_mode(ParseMode::MarkdownV2)
                .await
        }
        _ => bot.send_message(*chat_id, "Unknown button clicked").await,
    };
    wrap_error(result);
    println!("Message type: {}", callback_data.message_type);
}

async fn send_new_build_message(
    chat_id: &ChatId,
    bot: &Bot,
    message_id: &MessageId,
    text: &str,
) -> Result<Message, RequestError> {
    let has_photo = !current_build(chat_id)
        .await
        .photo_id
        .unwrap_or("".to_string())
        .as_str()
        .is_empty();

    if has_photo {
        bot.edit_message_caption(*chat_id, *message_id)
            .caption(text)
            .await
    } else {
        bot.edit_message_text(*chat_id, *message_id, text).await
    }
}

async fn hero_build_callback(
    data: &str,
    chat_id: &ChatId,
    bot: &Bot,
    message_id: &MessageId,
    username: &str,
) {
    let callback_data = CallbackData::from(data.to_string());

    println!("Message type: {}", callback_data.message_type);

    let repo = HeroBuildRepository::new();

    let result = match callback_data.button_type.as_str() {
        NEXT_BUTTON => {
            let incremented: usize = callback_data.incremented_index();
            send_update_navigation_media(chat_id, bot, message_id, username, &repo, incremented)
                .await
        }
        PREVIOUS_BUTTON => match callback_data.decremented_index() {
            Some(decrement) => {
                send_update_navigation_media(chat_id, bot, message_id, username, &repo, decrement)
                    .await
            }
            None => Result::Err(RequestError::MigrateToChatId(*chat_id)),
        },
        ADMIN_BUILD_BUTTON => {
            let current_build = current_build(chat_id).await;
            update_last_action(chat_id, ADMIN_BUILD_BUTTON).await;
            bot.send_message(*chat_id, current_build.text())
                .parse_mode(ParseMode::MarkdownV2)
                .reply_markup(new_build_keyboard(username, chat_id).await)
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

async fn send_update_navigation_media(
    chat_id: &ChatId,
    bot: &Bot,
    message_id: &MessageId,
    username: &str,
    repo: &HeroBuildRepository,
    index: usize,
) -> Result<Message, RequestError> {
    let hero_build = repo
        .find_build_by_index(index)
        .await
        .unwrap_or(HeroBuild::default());

    let message = hero_build.clone().text();
    if hero_build.file_as_input_media().is_some() {
        let input_media = InputMedia::Photo(InputMediaPhoto {
            media: hero_build.clone().input_file().unwrap(),
            caption: Some(message),
            parse_mode: Some(ParseMode::MarkdownV2),
            caption_entities: None,
            has_spoiler: false,
        });
        bot.edit_message_media(
            *chat_id,
            *message_id,
            input_media,
        )
        .reply_markup(hero_build_keyboard(index, username))
        .await
    } else {
        Err(RequestError::MigrateToChatId(*chat_id))
    }
}

struct CallbackData {
    button_type: String,
    message_type: String,
    index: usize,
}

impl CallbackData {
    fn from(message: String) -> CallbackData {
        println!("Trying to split message: {}", message);
        let (button, index) = message.split_once('-').unwrap();
        let (button, message_type) = button.split_once(':').unwrap();
        let index: usize = index
            .parse()
            .expect("Failed to parse index in CallbackData");

        CallbackData {
            button_type: button.to_string(),
            message_type: message_type.to_string(),
            index,
        }
    }

    fn incremented_index(&self) -> usize {
        self.index + 1
    }

    fn decremented_index(&self) -> Option<usize> {
        if self.index > 1 {
            Some(self.index - 1)
        } else {
            None
        }
    }
}
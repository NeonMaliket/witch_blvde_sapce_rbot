use crate::domain::repository::{current_build, last_action, STORAGE};
use crate::keyboards::new_build_keyboard;
use crate::message_buttons_handler::button_callback::new_build::{ADD_DESC, ADD_PHOTO, ADD_TITLE};
use crate::messages::MessageResponse;
use teloxide::prelude::*;
use teloxide::types::{InputFile, MediaKind, MessageKind, ParseMode};
use teloxide::RequestError;

pub async fn common_text_handler(bot: Bot, msg: Message) -> ResponseResult<()> {
    let chat_id = msg.chat.id;
    let text = msg.text().unwrap_or("").to_string();
    let photo_id = get_photo_id(&msg);
    println!("PHOTO ID: {}", photo_id.clone());
    let username = msg.from.unwrap().username.unwrap_or("".to_string()).clone();

    println!("TEXT: {}", text);
    let response_text = match new_build_text_handler(&chat_id, text, photo_id).await {
        Ok(value) => value,
        Err(value) => return value,
    };

    println!("Response text: {:?}", response_text);

    let current_build = current_build(&chat_id).await;
    if current_build.photo_id.is_some() {
        println!("Send with photo");
        if let Err(e) = bot
            .send_photo(chat_id, InputFile::file_id(current_build.photo_id.unwrap()))
            .parse_mode(ParseMode::MarkdownV2)
            .caption(response_text)
            .reply_markup(new_build_keyboard(username.as_str(), &chat_id).await)
            .await
        {
            eprintln!("Failed to send message: {:?}", e);
        }
    } else {
    if let Err(e) = bot
        .send_message(chat_id, response_text)
        .parse_mode(ParseMode::MarkdownV2)
        .reply_markup(new_build_keyboard(username.as_str(), &chat_id).await)
        .await
    {
        eprintln!("Failed to send message: {:?}", e);
    }
    }


    Ok(())
}

async fn new_build_text_handler(
    chat_id: &ChatId,
    text: String,
    photo_id: String,
) -> Result<String, ResponseResult<()>> {
    let (mut new_build, last_action) = {
        let new_build = current_build(chat_id).await;
        let last_action = last_action(chat_id).await;

        (new_build, last_action)
    };

    println!("last_action: {:?}", last_action);

    match last_action.as_str() {
        ADD_PHOTO => {
            new_build.photo_id = Some(photo_id);
            println!("COMMON TEXT (photo id): {}", text);
        }
        ADD_TITLE => {
            new_build.title = Some(text.clone());
            println!("COMMON TEXT (title): {}", text);
        }
        ADD_DESC => {
            new_build.description = Some(text.clone());
            println!("COMMON TEXT (description): {}", text);
        }
        _ => {
            println!("Last action text not found");
        }
    }

    let response_message_text = {
        let mut storage = STORAGE.lock().await;
        let m = new_build.text();
        storage.new_build(chat_id, new_build);
        m
    };

    Ok(response_message_text)
}

pub fn get_photo_id(msg: &Message) -> String {
    if let MessageKind::Common(ref common) = msg.kind {
        if let MediaKind::Photo(media) = &common.media_kind {
            return match media.photo.iter().last() {
                None => String::from(""),
                Some(photo_size) => photo_size.clone().file.id,
            };
        }
    }
    String::from("")
}

fn wrap_error(result: Result<Message, RequestError>) {
    if let Err(e) = result {
        println!("[ERROR]: [{}]", e);
    }
}

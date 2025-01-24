use crate::domain::repository::STORAGE;
use crate::message_buttons_handler::button_callback::new_build::{ADD_DESC, ADD_PHOTO, ADD_TITLE};
use teloxide::prelude::*;
use teloxide::RequestError;

pub async fn common_text_handler(bot: Bot, msg: Message) -> ResponseResult<()> {
    let chat_id = msg.chat.id;
    let text = msg.text().unwrap_or("").to_string(); // Конвертация в String один раз

    println!("TEXT: {}", text);
    let response_text = match new_build_text_handler(&chat_id, text).await {
        Ok(value) => value,
        Err(value) => return value,
    };

    println!("Response text: {:?}", response_text);

    if let Err(e) = bot.send_message(chat_id, response_text).await {
        eprintln!("Failed to send message: {:?}", e);
    }

    Ok(())
}

async fn new_build_text_handler(chat_id: &ChatId, text: String) -> Result<String, ResponseResult<()>> {
    let (mut new_build, last_action) = {

        let new_build = {
            match STORAGE.lock().await.get_build(chat_id).cloned() {
                Some(build) => build,
                _ => {
                    println!("New build not found");
                    return Err(Ok(()));
                }
            }
        };

        let last_action = {
            match STORAGE.lock().await.get_last_action_and_remove(chat_id) {
                Some(action) if !action.is_empty() => action.to_string().clone(),
                _ => {
                    println!("No last action found");
                    return Err(Ok(()));
                }
            }
        };

        (new_build, last_action)
    };

    println!("last_action: {:?}", last_action);

    match last_action.as_str() {
        ADD_PHOTO => {
            new_build.photo_url = Some(text.clone());
            println!("COMMON TEXT (photo): {}", text);
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

    {
        let mut storage = STORAGE.lock().await;
        storage.new_build(chat_id, new_build);
    }

    Ok(last_action.to_string())
}

fn wrap_error(result: Result<Message, RequestError>) {
    if let Err(e) = result {
        println!("[ERROR]: [{}]", e);
    }
}

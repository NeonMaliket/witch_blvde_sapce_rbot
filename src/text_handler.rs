use crate::domain::repository::STORAGE;
use crate::message_buttons_handler::button_callback::new_build::{ADD_DESC, ADD_PHOTO, ADD_TITLE};
use teloxide::prelude::*;
use teloxide::RequestError;

pub async fn common_text_handler(
    bot: Bot,
    msg: Message,
) -> ResponseResult<()> {
    let chat_id = msg.chat.id;
    let text = msg.text();

    println!("TEXT: {}", text.unwrap());
    let last_action_text = {
        let last_action = STORAGE.lock().unwrap();
        last_action
            .get_last_action(&chat_id)
            .unwrap()
            .to_owned()
    };

    println!("{:?}", last_action_text);

    print!("COMMON TEXT: {}", msg.text().unwrap());

        if last_action_text.eq(ADD_PHOTO) {
            print!("COMMON TEXT: {}", msg.text().unwrap());
            bot.send_message(chat_id, "ADD_PHOTO").await?;
        }
        if last_action_text.eq(ADD_TITLE) {
            print!("COMMON TEXT: {}", msg.text().unwrap());
            bot.send_message(chat_id, "ADD_TITLE").await?;
        }
        if last_action_text.eq(ADD_DESC) {
            print!("COMMON TEXT: {}", msg.text().unwrap());
            bot.send_message(chat_id, "ADD_DESC").await?;
        }

    Ok(())
}

fn wrap_error(result: Result<Message, RequestError>) {
    if let Err(e) = result {
        println!("[ERROR]: [{}]", e);
    }
}
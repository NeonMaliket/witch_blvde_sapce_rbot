use crate::message_buttons_handler::button_callback::*;
use crate::message_buttons_handler::message_type::HERO_BUILDS;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

pub fn hero_build_keyboard(page_index: u32) -> InlineKeyboardMarkup {
    let keyboard = InlineKeyboardMarkup::new(vec![
        vec![
            InlineKeyboardButton::callback(
                "<",
                update_hero_build_with_index(PREVIOUS_BUTTON, page_index),
            ),
            InlineKeyboardButton::callback(
                ">",
                update_hero_build_with_index(NEXT_BUTTON, page_index),
            ),
        ],
        vec![InlineKeyboardButton::callback("(Admin) Add New Build", update_hero_build_with_index(
            ADMIN_BUILD_BUTTON, page_index,
        ))],
        vec![InlineKeyboardButton::callback("Share self build", update_hero_build_with_index(
            SHARE_BUILD_BUTTON, page_index,
        ))],
    ]);
    keyboard
}

fn update_hero_build_with_index(callback: &str, index: u32) -> String {
    format!("{}:{}-{}", callback, HERO_BUILDS, index)
}

use crate::message_buttons_handler::button_callback::hero_builds::*;
use crate::message_buttons_handler::button_callback::new_build::*;
use crate::message_buttons_handler::message_type::{HERO_BUILDS, NEW_BUILD};
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};
use crate::domain::repository::is_admin;

pub fn hero_build_keyboard(page_index: u32, username: &str) -> InlineKeyboardMarkup {
    let new_build_keyboard = if is_admin(&username) {
        vec![InlineKeyboardButton::callback(
            "(Admin) Add New Build",
            update_hero_build_with_index(ADMIN_BUILD_BUTTON, page_index),
        )]
    } else {
        vec![]
    };

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
        new_build_keyboard,
        vec![InlineKeyboardButton::callback(
            "Share self build",
            update_hero_build_with_index(SHARE_BUILD_BUTTON, page_index),
        )],
    ]);
    keyboard
}

pub fn new_build_keyboard(username: &str) -> InlineKeyboardMarkup {
    if !is_admin(&username) {
        return InlineKeyboardMarkup::new(vec![vec![]]);
    }
    let keyboard = InlineKeyboardMarkup::new(vec![
        vec![InlineKeyboardButton::callback(
            "Add Photo",
            update_new_build_with_index(ADD_PHOTO),
        )],
        vec![InlineKeyboardButton::callback(
            "Add Title",
            update_new_build_with_index(ADD_TITLE),
        )],
        vec![InlineKeyboardButton::callback(
            "Add Description",
            update_new_build_with_index(ADD_DESC),
        )],
        vec![InlineKeyboardButton::callback(
            "Save Build",
            update_new_build_with_index(SAVE_BUILD),
        )],
    ]);
    keyboard
}



fn update_new_build_with_index(callback: &str) -> String {
    format!("{}:{}-0", callback, NEW_BUILD)
}

fn update_hero_build_with_index(callback: &str, index: u32) -> String {
    format!("{}:{}-{}", callback, HERO_BUILDS, index)
}

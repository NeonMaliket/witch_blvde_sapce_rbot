use crate::domain::repository::{current_build, is_admin};
use crate::message_buttons_handler::button_callback::hero_builds::*;
use crate::message_buttons_handler::button_callback::new_build::*;
use crate::message_buttons_handler::message_type::{HERO_BUILDS, NEW_BUILD};
use teloxide::prelude::ChatId;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

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

pub async fn new_build_keyboard(username: &str, chat_id: &ChatId) -> InlineKeyboardMarkup {
    if !is_admin(&username) {
        return InlineKeyboardMarkup::new(vec![vec![]]);
    }
    let current_build = current_build(&chat_id).await;
    let photo_title = current_build.clone()
        .photo_id
        .map(|_| "Update Photo")
        .unwrap_or("Add Photo");
    let title = current_build.clone()
        .title
        .map(|_| "Update Title")
        .unwrap_or("Add Title");
    let description_title = current_build.clone()
        .description
        .map(|_| "Update Description")
        .unwrap_or("Add Description");
    let save_build_keyboard = {
      if current_build.photo_id.is_some() && current_build.title.is_some() && current_build.description.is_some() {
          vec![InlineKeyboardButton::callback(
              "Save Build",
              update_new_build_with_index(SAVE_BUILD),
          )]
      } else {
          vec![]
      }
    };
    let keyboard = InlineKeyboardMarkup::new(vec![
        vec![InlineKeyboardButton::callback(
            photo_title,
            update_new_build_with_index(ADD_PHOTO),
        )],
        vec![InlineKeyboardButton::callback(
            title,
            update_new_build_with_index(ADD_TITLE),
        )],
        vec![InlineKeyboardButton::callback(
            description_title,
            update_new_build_with_index(ADD_DESC),
        )],
        save_build_keyboard,
    ]);
    keyboard
}

fn update_new_build_with_index(callback: &str) -> String {
    format!("{}:{}-0", callback, NEW_BUILD)
}

fn update_hero_build_with_index(callback: &str, index: u32) -> String {
    format!("{}:{}-{}", callback, HERO_BUILDS, index)
}

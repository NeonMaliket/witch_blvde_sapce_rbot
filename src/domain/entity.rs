use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use teloxide::prelude::ChatId;
use teloxide::types::{InputFile, InputMedia, InputMediaPhoto};

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct HeroBuild {
    pub id: Option<u64>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub photo_id: Option<String>,
}

impl HeroBuild {
    pub fn input_file(&self) -> Option<InputFile> {
        self.clone().photo_id.map(|id| InputFile::file_id(id))
    }

    pub fn file_as_input_media(&self) -> Option<InputMedia> {
        self.clone()
            .input_file()
            .map(|input_file| InputMedia::Photo(InputMediaPhoto::new(input_file)))
    }
}

#[derive(Debug, Default)]
pub struct LocalStorage<'a> {
    new_builds: HashMap<ChatId, HeroBuild>,
    last_action: HashMap<ChatId, &'a str>,
}

impl<'a> LocalStorage<'a> {
    pub fn default_build_for(&mut self, id: ChatId) {
        self.new_builds.entry(id).or_insert(HeroBuild::default());
    }

    pub fn new_build(&mut self, id: &ChatId, build: HeroBuild) {
        self.new_builds.entry(*id).insert_entry(build);
    }

    pub fn get_build(&mut self, id: &ChatId) -> Option<&mut HeroBuild> {
        self.new_builds.get_mut(id)
    }

    pub fn update_last_action(&mut self, id: ChatId, action: &'a str) {
        self.last_action.insert(id, action);
    }

    pub fn remove_last_action(&mut self, id: &ChatId) {
        self.last_action.remove(id);
    }

    pub fn get_last_action(&self, id: &ChatId) -> Option<&str> {
        self.last_action.get(id).map(|s| *s)
    }
}

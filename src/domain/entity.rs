use std::collections::HashMap;
use teloxide::prelude::ChatId;

#[derive(Clone, Debug, Default)]
pub struct HeroBuild {
    pub title: Option<String>,
    pub description: Option<String>,
    pub photo_url: Option<String>,
}

impl HeroBuild {
    pub(crate) fn new(title: &str, description: &str, photo_url: &str) -> Self {
        HeroBuild {
            title: Some(title.to_string()),
            description: Some(description.to_string()),
            photo_url: Some(photo_url.to_string()),
        }
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

    pub fn get_last_action_and_remove(&mut self, id: &ChatId) -> Option<&str> {
        let option = self.last_action.get(id).map(|s| *s);
        self.last_action.remove(id);
        option
    }

    pub fn get_last_action(&self, id: &ChatId) -> Option<&str> {
        self.last_action.get(id).map(|s| *s)
    }

}

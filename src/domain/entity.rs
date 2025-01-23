use std::collections::HashMap;
use teloxide::prelude::ChatId;

#[derive(Clone, Copy, Debug, Default)]
pub struct HeroBuild<'a> {
    pub title: Option<&'a str>,
    pub description: Option<&'a str>,
    pub photo_url: Option<&'a str>,
}

impl<'a> HeroBuild<'a> {
    pub(crate) fn new(title: &'a str, description: &'a str, photo_url: &'a str) -> Self {
        HeroBuild {
            title: Some(title),
            description: Some(description),
            photo_url: Some(photo_url),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct LocalStorage<'a> {
    new_builds: HashMap<ChatId, HeroBuild<'a>>,
    last_action: HashMap<ChatId, &'a str>,
}

impl<'a> LocalStorage<'a> {
    pub fn default_build_for(&mut self, id: ChatId) {
        self.new_builds.entry(id).or_insert(HeroBuild::default());
    }

    pub fn get_build(&self, id: &'a ChatId) -> Option<&HeroBuild<'a>> {
        self.new_builds.get(id)
    }

    pub fn update_last_action(&mut self, id: ChatId, action: &'a str) {
        self.last_action.insert(id, action);
    }

    pub fn get_last_action(&self, id: &'a ChatId) -> Option<&str> {
        self.last_action.get(id).map(|s| *s)
    }
}

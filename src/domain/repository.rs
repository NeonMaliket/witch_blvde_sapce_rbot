use crate::domain::entity::{HeroBuild, LocalStorage};
use crate::domain::supabase::get_supabase_config;
use once_cell::sync::Lazy;
use serde_json::{from_value, json};
use supabase_rs::SupabaseClient;
use teloxide::prelude::ChatId;
use tokio::sync::Mutex;

type BotStorage = Lazy<Mutex<LocalStorage<'static>>>;

lazy_static::lazy_static! {
    pub static ref STORAGE: BotStorage = Lazy::new(|| Mutex::new(LocalStorage::default()));
}

#[derive(Clone)]
pub(crate) struct HeroBuildRepository {
    client: &'static SupabaseClient,
    table_name: &'static str,
}

impl HeroBuildRepository {
    pub(crate) fn new() -> Self {
        Self {
            client: get_supabase_config(),
            table_name: "hero_builds",
        }
    }

    pub(crate) async fn save(&mut self, build: HeroBuild) -> Result<(), String> {
        println!("Saving hero build: {:#?}", build);
        self.client
            .insert(
                self.table_name,
                json!(build),
            )
            .await
            .expect("Could not add hero_builds");

        Ok(())
    }

    pub(crate) async fn find_all_builds(&self) -> Vec<HeroBuild> {
        let data: Vec<HeroBuild> = self.client
            .select(self.table_name)
            .columns(["id", "title", "description", "photo_id"].to_vec())
            .execute()
            .await.unwrap_or(vec![]).into_iter()
            .map(|item| {
                from_value(item)
                    .map_err(|e| format!("Error deserializing: {}", e))
            })
            .collect::<Result<_, _>>().unwrap_or(vec![]);

        data
    }

    pub(crate) fn find_first_build(&self) -> Option<HeroBuild> {
        self.find_build_by_index(1)
    }

    pub(crate) fn find_build_by_index(&self, index: u32) -> Option<HeroBuild> {
        None
    }
}

pub async fn current_build(chat_id: &ChatId) -> HeroBuild {
    STORAGE
        .lock()
        .await
        .get_build(chat_id)
        .cloned()
        .unwrap_or(HeroBuild::default())
}

pub async fn last_action(chat_id: &ChatId) -> String {
    STORAGE
        .lock()
        .await
        .get_last_action_and_remove(chat_id)
        .map(str::to_string)
        .unwrap_or(String::default())
}

pub fn is_admin(username: &str) -> bool {
    let admins = vec!["Ma1iket"];
    println!("username: {}", username);
    admins.contains(&username)
}

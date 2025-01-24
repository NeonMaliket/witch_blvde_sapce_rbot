use crate::domain::entity::{HeroBuild, LocalStorage};
use once_cell::sync::Lazy;
use tokio::sync::Mutex;

type BotStorage = Lazy<Mutex<LocalStorage<'static>>>;

lazy_static::lazy_static! {
    pub static ref STORAGE: BotStorage = Lazy::new(|| Mutex::new(LocalStorage::default()));
}

#[derive(Clone)]
pub(crate) struct HeroBuildRepository {
    store: Vec<HeroBuild>,
}

impl HeroBuildRepository {
    pub(crate) fn new() -> Self {
        Self { store: vec![] }
    }

    #[warn(unused_unsafe)]
    pub(crate) fn save(&mut self, build: HeroBuild) -> Result<(), String> {
        println!("Saving hero build: {:#?}", build);
        self.store.push(build);
        Ok(())
    }

    pub(crate) fn find_all_builds(&self) -> Vec<HeroBuild> {
        self.store.clone()
    }

    pub(crate) fn find_first_build(&self) -> Option<HeroBuild> {
        self.find_build_by_index(1)
    }

    pub(crate) fn find_build_by_index(&self, index: u32) -> Option<HeroBuild> {
        if self.store.len() < index as usize {
            None
        } else {
            let mut index: usize = index as usize;
            if index > 0 {
                index = index - 1;
            }
            Some(self.store[index].clone())
        }
    }
}

pub fn is_admin(username: &str) -> bool {
    let admins = vec!["MALlKETH"];
    println!("username: {}", username);
    admins.contains(&username)
}
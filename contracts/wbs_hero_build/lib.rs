#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod wbs_hero_build {
    use ink::prelude::string::String;

    #[ink(storage)]
    pub struct HeroBuild {
        id: u64,
        title: String,
        description: String,
        photo_id: String,
        created_on: u64,
    }

    impl HeroBuild {
        #[ink(constructor)]
        pub fn new(id: u64, title: String, description: String, photo_id: String) -> Self {
            Self {
                id,
                title,
                description,
                photo_id,
                created_on: Self::env().block_timestamp(),
            }
        }

        #[ink(message)]
        pub fn update_title(&mut self, title: String) {
            self.title = title;
        }

        #[ink(message)]
        pub fn update_description(&mut self, description: String) {
            self.description = description;
        }

        #[ink(message)]
        pub fn update_photo_id(&mut self, photo_id: String) {
            self.photo_id = photo_id;
        }

        #[ink(message)]
        pub fn created_on(&self) -> u64 {
            self.created_on
        }

        #[ink(message)]
        pub fn account_id(&self) -> AccountId {
            Self::env().account_id()
        }

        #[ink(message)]
        pub fn get(&self) -> (u64, String, String, String) {
            (
                self.id,
                self.title.clone(),
                self.description.clone(),
                self.photo_id.clone(),
            )
        }
    }
}

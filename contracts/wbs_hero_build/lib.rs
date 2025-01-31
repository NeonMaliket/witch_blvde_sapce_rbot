#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod wbs_hero_build {
    use ink::prelude::string::String;
    use ink::storage::Mapping;
    use ink::storage::StorageVec;
    use ink::prelude::vec::Vec;

    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[derive(Clone)]
    pub struct SingleBuild {
        id: u64,
        title: String,
        description: String,
        photo_id: String,
        created_on: u64,
    }

    impl SingleBuild {
        pub fn new(
            id: u64,
            title: String,
            description: String,
            photo_id: String,
            created_on: u64,
        ) -> Self {
            Self {
                id,
                title,
                description,
                photo_id,
                created_on,
            }
        }
    }

    #[ink(storage)]
    #[derive(Default)]
    pub struct HeroBuilds {
        builds: Mapping<AccountId, SingleBuild>,
        array: StorageVec<SingleBuild>,
    }

    impl HeroBuilds {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                builds: Mapping::default(),
                array: StorageVec::default(),
            }
        }

        #[ink(message)]
        pub fn add_build(&mut self, title: String, description: String, photo_id: String) {
            let caller = self.env().caller();
            let build = &SingleBuild::new(
                Self::env().block_timestamp(),
                title,
                description,
                photo_id,
                Self::env().block_timestamp(),
            );
            self.builds.insert(caller, build);
            self.array.push(build);
        }

        #[ink(message)]
        pub fn get_as_array(&mut self) -> Vec<SingleBuild> {
            let mut vec = Vec::new();

            while let Some(hero) = self.array.pop() {
                vec.push(hero);
            }

            vec
        }

        #[ink(message)]
        pub fn get_single(&self) -> Option<SingleBuild> {
            let caller = self.env().caller();
            let result = self.builds.get(caller)?.clone();
            Some(result)
        }
    }
}

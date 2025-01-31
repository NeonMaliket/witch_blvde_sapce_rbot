#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod wbs_hero_build {
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    use ink::storage::StorageVec;

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
        builds: StorageVec<SingleBuild>,
    }

    impl HeroBuilds {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                builds: StorageVec::default(),
            }
        }

        #[ink(message)]
        pub fn add_build(
            &mut self,

            title: String,
            description: String,
            photo_id: String,
        ) {
            self.builds.push(&SingleBuild::new(
                Self::env().block_timestamp(),
                title,
                description,
                photo_id,
                Self::env().block_timestamp(),
            ));
        }

        #[ink(message)]
        pub fn get_as_array(&mut self) -> Vec<SingleBuild> {
            let mut vec = Vec::new();

            while let Some(hero) = self.builds.pop() {
                vec.push(hero);
            }

            vec
        }
    }
}

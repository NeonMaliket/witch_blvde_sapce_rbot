use crate::domain::entity::HeroBuild;

#[derive(Clone)]
pub(crate) struct HeroBuildRepository<'a> {
    store: Vec<HeroBuild<'a>>,
}

impl<'h> HeroBuildRepository<'h> {
    pub(crate) fn new() -> Self {
        Self { store: vec![] }
    }

    #[warn(unused_unsafe)]
    pub(crate) fn save(&mut self, build: HeroBuild<'h>) -> Result<(), String> {
        println!("Saving hero build: {:#?}", build);
        self.store.push(build);
        Ok(())
    }

    pub(crate) fn find_all_builds(&self) -> Vec<HeroBuild<'_>> {
        self.store.clone()
    }

    pub(crate) fn find_first_build(&self) -> Option<HeroBuild<'h>> {
        self.find_build_by_index(1)
    }

    pub(crate) fn find_build_by_index(&self, index: u32) -> Option<HeroBuild<'h>> {
        if self.store.len() < index as usize {
            None
        } else {
            let mut index: usize = index as usize;
            if index > 0 {
                index = index - 1;
            }
            Some(self.store[index])
        }
    }
}

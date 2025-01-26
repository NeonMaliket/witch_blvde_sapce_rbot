pub(crate) trait MessageResponse {
    fn text(&self) -> String;
}

pub(crate) struct BuildNotFoundMessageResponse;

impl MessageResponse for BuildNotFoundMessageResponse {
    fn text(&self) -> String {
        String::from("Build not found")
    }
}

pub(crate) mod hero_build {
    use crate::domain::entity::HeroBuild;
    use crate::messages::MessageResponse;

    impl MessageResponse for HeroBuild {
        fn text(&self) -> String {
            let hero_build = self.clone();

            let message = format!(
                "\
            *Hero Build*\n\
            Title: {}\n\
            Description: {}\n",
                hero_build.title.unwrap_or(String::from("")).as_str(),
                hero_build.description.unwrap_or(String::from("")).as_str(),
            );

            message
        }
    }
}

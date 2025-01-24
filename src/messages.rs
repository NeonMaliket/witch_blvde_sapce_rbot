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
            let mut message = String::from("Hero Build");

            self.clone().title.map(|title| {
                message.push('\n');
                message.push_str(title.as_str());
            });
            self.clone().description.map(|desc| {
                message.push('\n');
                message.push_str(desc.as_str());
                message.push('\n');
            });
            self.clone().photo_url.map(|url| {
                message.push_str(url.as_str());
                message.push('\n');
            });
            message
        }
    }
}

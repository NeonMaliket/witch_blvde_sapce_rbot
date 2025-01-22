#[derive(Clone, Copy, Debug)]
pub(crate) struct HeroBuild<'a> {
    pub(crate) title: &'a str,
    pub(crate) description: Option<&'a str>,
    pub(crate) photo_url: Option<&'a str>,
}

impl<'a> HeroBuild<'a> {
    pub(crate) fn new(title: &'a str, description: &'a str, photo_url: &'a str) -> Self {
        HeroBuild {
            title,
            description: Some(description),
            photo_url: Some(photo_url),
        }
    }
}

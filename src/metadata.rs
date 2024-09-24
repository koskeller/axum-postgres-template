pub struct Metadata {
    pub name: String,
    pub title: String,
    pub description: String,
    pub base_url: String,
    pub canonical: String,
    pub lang: String,
    pub r#type: String,
}

pub struct OpenGraph {
    pub title: String,
    pub description: String,
    pub url: String,
    pub site_name: String,
    pub images: Vec<Image>,
    pub locale: String,
    pub r#type: String,
}

pub struct Image {
    pub url: String,
    pub width: i32,
    pub height: i32,
}

impl Metadata {
    pub fn default() -> Self {
        Self {
            name: "".to_string(),
            title: "".to_string(),
            description: "".to_string(),
            base_url: "".to_string(),
            canonical: "/".to_string(),
            lang: "en".to_string(),
            r#type: "Website".to_string(),
        }
    }
}

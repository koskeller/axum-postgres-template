use std::env::var;

#[derive(serde::Deserialize)]
pub struct Settings {
    pub db_url: String,
    pub db_name: String,
    pub app_port: u16,
}

impl Settings {
    pub fn new() -> anyhow::Result<Self> {
        let app_port = var("PORT")?.parse::<u16>()?;
        let db_url = var("DATABASE_URL")?;
        let db_name = var("DATABASE_NAME")?;

        Ok(Settings {
            db_url,
            db_name,
            app_port,
        })
    }

    pub fn db_url_without_db(&self) -> String {
        self.db_url.replace(&self.db_name, "")
    }
}

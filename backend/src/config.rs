use serde::Deserialize;
use dotenv::dotenv;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub github_app: Option<GithubApp>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GithubApp {
    #[serde(rename = "GITHUB_APP_ID")]
    pub id: String,
    pub secret: String,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        dotenv().ok();

        let app_id = std::env::var("GITHUB_APP_ID")?;
        let secret = fs::read_to_string("private-key.pem")?;

        Ok(Config {
            github_app: Some(GithubApp {
                id: app_id,
                secret,
            })
        })
    }
}

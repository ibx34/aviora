use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Server {
    pub postgres_uri: String,
    pub fivem_key: String,
    pub unique_state_id_name: Option<UniqueStateIdName>,
    #[serde(default)]
    pub allow_identity_theft: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UniqueStateIdName {
    pub shorthand: Option<String>,
    pub longhand: Option<String>,
    pub format: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub server: Server,
}

pub const ALLOWED_RACES: [&str; 3] = ["white", "black", "mixed"];
pub const CONFIG: Lazy<Config> = Lazy::new(|| {
    toml::from_str::<Config>(&std::fs::read_to_string("./Config.toml").unwrap()).unwrap()
});

use crate::{
    config::{Config, CONFIG},
    db::Database,
};
use anyhow::Result;
use sha2::{Digest, Sha256};
use snowflake::SnowflakeIdGenerator;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct App {
    pub config: Config,
    pub snowflakes: Arc<Mutex<SnowflakeIdGenerator>>,
    pub db: Database,
}

impl App {
    pub async fn init() -> Result<Self> {
        let config = CONFIG.to_owned();
        let snowflakes = Arc::new(Mutex::new(SnowflakeIdGenerator::new(0, 1)));

        Ok(Self {
            db: Database::init(config.server.postgres_uri).await?,
            config: CONFIG.to_owned(),
            snowflakes,
        })
    }
    pub async fn generate_id(&self) -> i64 {
        let cloned_snowflakes = self.snowflakes.clone();
        let mut lock = cloned_snowflakes.lock().await;
        lock.generate()
    }
    pub async fn generate_civ_ref(&self, first_name: &str, last_name: &str) -> (i64, String) {
        let id = self.generate_id().await;
        let mut hasher = Sha256::new();
        hasher.update(id.to_string().as_bytes());
        hasher.update(first_name.as_bytes());
        hasher.update(last_name.as_bytes());

        let user_ref = hasher.finalize();
        (id, hex::encode(user_ref))
    }
}

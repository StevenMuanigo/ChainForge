use super::AppConfig;
use std::sync::Arc;
use tokio::sync::RwLock;

pub type SharedConfig = Arc<RwLock<AppConfig>>;

pub fn create_shared_config(config: AppConfig) -> SharedConfig {
    Arc::new(RwLock::new(config))
}

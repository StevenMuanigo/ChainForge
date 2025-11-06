use crate::config::AppConfig;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use tracing_appender::rolling::{RollingFileAppender, Rotation};

pub fn init_logger(config: &AppConfig) {
    let log_level = &config.monitoring.log_level;
    let log_format = &config.monitoring.log_format;
    
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(log_level));
    
    let file_appender = RollingFileAppender::new(Rotation::DAILY, "./logs", "chainforge.log");
    
    if log_format == "json" {
        let file_layer = tracing_subscriber::fmt::layer()
            .json()
            .with_writer(file_appender);
        
        let stdout_layer = tracing_subscriber::fmt::layer()
            .json()
            .with_writer(std::io::stdout);
        
        tracing_subscriber::registry()
            .with(env_filter)
            .with(file_layer)
            .with(stdout_layer)
            .init();
    } else {
        let file_layer = tracing_subscriber::fmt::layer()
            .pretty()
            .with_writer(file_appender);
        
        let stdout_layer = tracing_subscriber::fmt::layer()
            .pretty()
            .with_writer(std::io::stdout);
        
        tracing_subscriber::registry()
            .with(env_filter)
            .with(file_layer)
            .with(stdout_layer)
            .init();
    }
    
    tracing::info!("Logger initialized with level: {}", log_level);
}

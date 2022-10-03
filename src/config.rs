use clap::Parser;
use crate::error::{Error, Result};

#[derive(Debug, Clone, clap::Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Config {
    #[clap(
        short,
        long,
        env = "INTELLIGENCCE_ADDR",
        default_value = "127.0.0.1:1433",
        help = "Address to listen on"
    )]
    pub address: String,

    #[clap(
        short,
        long,
        env = "INTELLIGENCCE_DB",
        default_value = "postgres://postgres:postgres@localhost:5432/intelligencce",
        help = "Database connection address"
    )]
    pub db: String,

    #[clap(
        long,
        help = "Expose insecure endpoints (like the GraphQL playground)"
    )]
    pub insecure: bool,

    #[clap(
        short,
        long = "templates",
        env = "INTELLIGENCCE_TEMPLATES",
        help = "Templates directory"
    )]
    templates_: Option<String>,
    #[clap(skip)]
    pub templates: String,
}
impl Config {
    pub fn new() -> Result<Self> {
        Self::load_env();
        Self::init_logger()?;
        let mut cfg = Self::parse();
        Self::check_templates(&mut cfg);

        log::info!("{} v{}", std::env::var("CARGO_PKG_NAME").unwrap_or("intelligencce".to_string()), std::env::var("CARGO_PKG_VERSION").unwrap_or("0.0.0".to_string()));
        Ok(cfg)
    }

    fn load_env() {
        dotenv::dotenv().ok();
    }

    fn init_logger() -> Result<()> {
        // pattern
        let pattern = log4rs::encode::pattern::PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S)} | {h({l}):5.5} | {m}{n}");
        // appenders
        let console = log4rs::config::Appender::builder()
            .build("console", Box::new(log4rs::append::console::ConsoleAppender::builder().encoder(Box::new(pattern)).build()));
        // configurations
        let root = log4rs::config::Root::builder()
            .appender("console")
            .build(log::LevelFilter::Debug);
        let config = log4rs::config::Config::builder()
            .appender(console)
            .build(root).map_err(|e| Error::LoggerError(e.to_string()))?;
        let _ = log4rs::init_config(config);
        Ok(())
    }

    fn check_templates(&mut self) {
        // check the templates directory
        if let Some(ref path) = self.templates_ {
            self.templates = path.to_string();
        } else if let Some(path) = dirs::config_dir() {
            if let Some(cofgpath) = path.join("intelligencce/templates").to_str() {
                self.templates = String::from(cofgpath);
                let _ = std::fs::create_dir_all(&self.templates); // Fixme: handle error
            }
        }
        if self.templates.is_empty() {
            self.templates = "./".to_string();
        }
    }
}

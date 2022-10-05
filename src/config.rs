use crate::error::{Error, Result};
use clap::Parser;

fn banner() {
    println!(
        r#"
 _       _       _ _ _                               
(_)_ __ | |_ ___| | (_) __ _  ___ _ __   ___ ___ ___ 
| | '_ \| __/ _ \ | | |/ _` |/ _ \ '_ \ / __/ __/ _ \
| | | | | ||  __/ | | | (_| |  __/ | | | (_| (_|  __/
|_|_| |_|\__\___|_|_|_|\__, |\___|_| |_|\___\___\___|
                       |___/   nccgroup.com  v{}          
"#,
        env!("CARGO_PKG_VERSION")
    );

    log::warn!("Use with caution. You are responsible for your actions.");
    log::warn!("Developers assume no liability and are not responsible for any misuse or damage.");
    log::warn!("-------------------------------------------------------");
}

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

    #[clap(long, help = "Expose insecure endpoints (like the GraphQL playground)")]
    pub insecure: bool,

    #[clap(long = "bare", help = "Do not print the banner")]
    pub bare: bool,

    #[clap(long = "disable-server", help = "Do not start the web server")]
    pub noserver: bool,

    #[clap(long = "disable-templates", help = "Do not run any template")]
    pub notemplates: bool,

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
        // load the config
        Self::load_env();
        Self::init_logger()?;
        let mut cfg = Self::parse();
        Self::check_templates(&mut cfg);
        // print the banner
        if !cfg.bare {
            banner();
        }
        // return the config
        Ok(cfg)
    }

    fn load_env() {
        // load the .env file
        dotenv::dotenv().ok();
    }

    fn init_logger() -> Result<()> {
        // pattern
        let pattern = log4rs::encode::pattern::PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S)} | {h({l}):5.5} | {m}{n}",
        );
        // appenders
        let console = log4rs::config::Appender::builder().build(
            "console",
            Box::new(
                log4rs::append::console::ConsoleAppender::builder()
                    .encoder(Box::new(pattern))
                    .build(),
            ),
        );
        // loggers
        let logger = log4rs::config::Logger::builder()
            .appender("console")
            .additive(false)
            .build(
                std::env::var("CARGO_PKG_NAME").unwrap_or("intelligencce".to_string()),
                log::LevelFilter::Trace,
            );
        // configurations
        let root = log4rs::config::Root::builder().build(log::LevelFilter::Trace);
        let config = log4rs::config::Config::builder()
            .appender(console)
            .logger(logger)
            .build(root)
            .map_err(|e| Error::LoggerError(e.to_string()))?;
        let _ = log4rs::init_config(config);
        Ok(())
    }

    fn check_templates(&mut self) {
        if self.notemplates {
            return
        }

        if let Some(ref path) = self.templates_ {
            // check the templates directory
            self.templates = path.to_string();
        } else if let Some(path) = dirs::config_dir() {
            // check the default config directory
            if let Some(cofgpath) = path.join("intelligencce").join("templates").to_str() {
                // check the default config directory for the templates
                self.templates = String::from(cofgpath);
                let _ = std::fs::create_dir_all(&self.templates); // FIXME: handle error
            }
        }
        if self.templates.is_empty() {
            // set the current directory as the default templates directory
            self.templates = ".".to_string();
        }
    }
}

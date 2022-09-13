use clap::Parser;

#[derive(Debug, clap::Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Config {
    #[clap(
        long,
        env = "REDIS_URL",
        default_value = "redis://127.0.0.1:6379/",
        help = "Redis URL"
    )]
    pub redis: String,

    #[clap(
        short,
        long,
        env = "INTELLIGENCCE_TEMPLATES",
        help = "Templates directory"
    )]
    templates_: Option<String>,
    #[clap(skip)]
    pub templates: String,
}
impl Config {
    pub fn new() -> Self {
        dotenv::dotenv().ok();
        let mut cfg = Self::parse();
        // check the templates directory
        if let Some(ref path) = cfg.templates_ {
            cfg.templates = path.to_string();
        } else if let Some(path) = dirs::config_dir() {
            if let Some(cofgpath) = path.join("intelligencce/templates").to_str() {
                cfg.templates = String::from(cofgpath);
                let _ = std::fs::create_dir_all(&cfg.templates); // Fixme: handle error
            }
        }
        if cfg.templates.is_empty() {
            cfg.templates = "./".to_string();
        }
        // return the config
        cfg
    }
}
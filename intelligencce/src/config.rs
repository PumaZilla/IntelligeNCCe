use clap::Parser;

#[derive(Debug, clap::Parser)]
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
    pub fn new() -> Self {
        Self::load_env();
        let mut cfg = Self::parse();
        Self::check_templates(&mut cfg);
        cfg
    }

    fn load_env() {
        dotenv::dotenv().ok();
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

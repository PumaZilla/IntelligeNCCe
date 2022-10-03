mod config;
mod database;
mod error;
mod templates;
mod web;

/// The main entry point for the application.
#[tokio::main]
async fn main() -> () {
    if let Err(error) = start().await {
        eprintln!("[!] {}", error);
        std::process::exit(1);
    }
}

/// Starts the application and returns an error if one occurs.
async fn start() -> error::Result<()> {
    let cfg = config::Config::new()?;
    // load the templates
    let _templates = templates::load_all(&cfg.templates)?;
    // start the web server
    let db = database::establish_connection(&cfg.db)?;
    web::start(cfg, db.into()).await?;
    Ok(())
}

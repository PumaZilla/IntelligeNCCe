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
    // connect to the database
    let db = database::establish_connection(&cfg.db)?;
    let shared_db = std::sync::Arc::new(db);
    // load the templates
    let templates = templates::load_all(&cfg.templates)?;
    // start the templates and the web server
    futures::join!(templates.start(shared_db.clone()), web::start(cfg, shared_db));
    Ok(())
}

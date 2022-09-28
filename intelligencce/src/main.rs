mod config;

async fn start() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = config::Config::new();
        
    /*
    let templates = intelligencce_templates::load_all(&cfg.templates)?;

    let (tx, rx) = std::sync::mpsc::channel();
    let atx = std::sync::Arc::new(std::sync::Mutex::new(tx));

    for watcher in templates.get_watchers() {
        let atx = std::sync::Arc::clone(&atx);
        std::thread::spawn(move || watcher.start(atx));
    }

    for data in rx {
        println!("{:#?}", data);
    }
    */

    intelligencce_server::start(&cfg.address, &cfg.db).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> () {
    match start().await {
        Ok(_) => (),
        Err(e) => {
            eprintln!("[!] {}", e);
            std::process::exit(1);
        }
    }
}

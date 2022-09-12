mod config;
mod templates;

#[tokio::main]
async fn main() {
    match entrypoint().await {
        Ok(_) => (),
        Err(e) => {
            eprintln!("[!] {}", e);
            std::process::exit(1);
        }
    }
}

async fn entrypoint() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = config::Config::new()?;
    
    let cfg_sub = cfg.clone();
    let suscriber = std::thread::spawn(move || {
        let mut conn = cfg_sub.clone().storage.cache.get_connection().unwrap();
        let mut pubsub = conn.as_pubsub();
        pubsub.subscribe("source").unwrap();
        loop {
            println!("Waiting for messages...");
            let msg = pubsub.get_message().unwrap();
            println!("Received: {:?}", msg);
            std::thread::sleep(std::time::Duration::from_secs(10));
        }
    });
    
    let tmpls = templates::load_all("./test/templates")?;
    for (id, tmpl) in tmpls {
        println!("[*] Running {}...", id);
        tmpl.run(&cfg).await;
    }

    suscriber.join().unwrap();
    Ok(())
}

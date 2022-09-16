mod config;
/*
mod storage;

fn start() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = config::Config::new();
    let storage = storage::Storage::new(&cfg)?;

    // load the all the templates
    let templates = intelligencce_templates::load_all(&cfg.templates)?;

    // create the channel
    let (tx, rx) = std::sync::mpsc::channel();
    let atx = std::sync::Arc::new(std::sync::Mutex::new(tx));

    // iterate over all templates 
    for (id,tmpl) in templates {
        let atx = std::sync::Arc::clone(&atx);
        let wtmpl = tmpl.clone();
        // start the watcher in a thread and then print the template debug info
        std::thread::spawn(move || {
            wtmpl.watch(atx);
        });
        println!("[*] Running: {}", id);
    }

    // start the listenner
    for msg in rx {
        storage.publish(&msg)?;
    }

    Ok(())
}

*/

fn start() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = config::Config::new();

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

    Ok(())
}

fn main() -> () {
    match start() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("[!] {}", e);
            std::process::exit(1);
        }
    }
}

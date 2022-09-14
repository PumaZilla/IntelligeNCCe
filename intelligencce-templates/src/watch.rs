//use super::output;
use super::{step, Tx};

//const DEFAULT_STORE_KEY: &str = "default";

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Watch {
    every: String,
    #[serde(rename="steps")]
    _steps: Vec<step::Step>,
}
impl Watch {
    /// Start watching for changes and send the results to the given channel
    pub fn start(&self, tx: Tx) {
        loop {
            let guard = tx.lock().unwrap();
            let _ = (*guard).send(format!("sending time: {}", self.every)); // TODO: Handle error
            std::mem::drop(guard);
            std::thread::sleep(std::time::Duration::from_secs(2));
        }
    }
}

/*
impl Watch {
    pub async fn watch(&self, cfg: &config::Config) {
        let time: std::time::Duration = duration_string::DurationString::from_string(String::from(&self.every)).unwrap_or(duration_string::DurationString::new(std::time::Duration::from_secs(3600))).into();
        println!("{:?}", time); // TODO: Use the time

        let default_args: Vec<output::Output> = Vec::new(); // FIXME: This is a hack, dunno how to do it better, needs to live long enough
        let mut store = std::collections::HashMap::<String,Vec<output::Output>>::new();
        for step in &self.steps {
            let args: &Vec<output::Output> = store.get(&step.load.clone().unwrap_or(DEFAULT_STORE_KEY.to_string())).unwrap_or(&default_args);
            match step.run(&cfg,&args).await {
                Ok(res) => {
                    if let Some(store_key) = &step.save_as {
                        store.insert(store_key.clone(), res.clone());
                    }
                    store.insert(DEFAULT_STORE_KEY.to_string(), res)
                },
                Err(err) => {
                    println!("[!] {}", err);
                    break
                }
            };
        }
    }
}
*/

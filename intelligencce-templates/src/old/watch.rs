use super::{data, step, DataTx};

const DEFAULT_STORE_KEY: &str = ":: default";

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Watch {
    every: String,
    steps: Vec<step::Step>,
}
impl Watch {
    /// Start watching for changes and send the results to the given channel
    pub fn start(&self, id: &str, tx: DataTx) -> Result<(), Box<dyn std::error::Error>> {
        let sleep_time = duration_string::DurationString::from_string(self.every.clone())?.into();
        loop {
            let results = self.run(id)?; // TODO: Handle error
            let guard = tx.lock().unwrap();
            for result in results {
                let _ = (*guard).send(result); // TODO: Handle error
            }
            std::mem::drop(guard);
            std::thread::sleep(sleep_time);
        }
    }

    fn run(&self, id: &str) -> Result<Vec<data::Data>, Box<dyn std::error::Error>> {
        let mut empty_ctx = data::Data::default();
        empty_ctx.template = id.to_string();
        let mut store: std::collections::HashMap<String, Vec<data::Data>> =
            std::collections::HashMap::new();
        store.insert(DEFAULT_STORE_KEY.to_string(), vec![empty_ctx]); // FIXME: This is a hack, dunno how to do it better, needs to live long enough
        for step in &self.steps {
            let context = store
                .get(&step.load.clone().unwrap_or(DEFAULT_STORE_KEY.to_string())).ok_or("No store key found!")?;
            match step.run(&context) {
                Ok(data) => {
                    if let Some(store_key) = &step.save_as {
                        store.insert(store_key.clone(), data.clone());
                    }
                    store.insert(DEFAULT_STORE_KEY.to_string(), data);
                }
                Err(e) => {
                    println!("Error: {}", e);
                    break;
                }
            }
        }

        Ok(store.get(DEFAULT_STORE_KEY).unwrap().clone())
    }
}

use super::output;
use super::step;
use crate::config;

const DEFAULT_STORE_KEY: &str = "default";

#[derive(Debug,serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Watch {
    pub every: String, // TODO: Change me to fit a real duration
    pub steps: Vec<step::Step>,
}
impl Watch {
    pub async fn watch(&self, cfg: &config::Config) {
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
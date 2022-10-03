use crate::error::{Error, Result};

const DEFAULT_KEY: &str = " ::default";

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TemplateWatcher {
    pub id: String,
    pub every: String,
    pub steps: Vec<super::step::TemplateStep>,
}
impl TemplateWatcher {
    pub fn from_template(template: &str) -> Result<Self> {
        Ok(serde_yaml::from_str(&template).map_err(|e| Error::TemplateParseError(template.to_string(), e.to_string()))?)
    }

    fn run(&self) -> Vec<super::data::Data> {
        let mut contents: Vec<super::data::Data> = Vec::new();
        let mut contexts: std::collections::HashMap<String, Option<Vec<super::data::Data>>> =
            std::collections::HashMap::new();
        contexts.insert(DEFAULT_KEY.to_string(), None);
        self.steps.iter().for_each(|step| {
            let (context, mut content) = step.run_multiple(
                contexts
                    .get(&step.load.clone().unwrap_or(DEFAULT_KEY.to_string()))
                    .unwrap()
                    .clone(),
            );
            contents.append(&mut content);
            let new_context = match context.is_empty() {
                true => None,
                false => Some(context),
            };
            if let Some(key) = &step.save_as {
                contexts.insert(key.clone(), new_context.clone());
            }
            contexts.insert(DEFAULT_KEY.to_string(), new_context);
        });
        contents.append(
            &mut contexts
                .get(DEFAULT_KEY)
                .unwrap()
                .clone()
                .unwrap_or_default(),
        );
        contents
    }

    pub fn start(
        &self,
        tx: std::sync::Arc<std::sync::Mutex<std::sync::mpsc::Sender<super::data::Data>>>,
    ) {
        println!(
            "[*] Starting watcher {} (triggered every {})...",
            &self.id, &self.every
        );
        match humantime::parse_duration(&self.every) {
            Ok(duration) => {
                loop {
                    println!("[*] Running watcher {}...", self.id);
                    let results: Vec<super::data::Data> = self
                        .run()
                        .iter()
                        .map(|data| {
                            let mut data = data.clone();
                            data.set_template(&self.id);
                            data
                        })
                        .collect();
                    match tx.lock() {
                        Ok(tx) => results.iter().for_each(|data| {
                            let _ = (*tx).send(data.to_owned()); // TODO: Handle error
                        }),
                        Err(err) => println!("[!] Cannot send data from {}: {}", &self.id, err),
                    };
                    std::thread::sleep(duration);
                }
            }
            Err(err) => println!("cannot parse duration from watcher {}: {}", &self.id, err),
        }
    }
}

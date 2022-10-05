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
        Ok(serde_yaml::from_str(&template)
            .map_err(|e| Error::TemplateParseError(template.to_string(), e.to_string()))?)
    }

    async fn run(&self) -> Vec<super::data::Data> {
        let mut contents: Vec<super::data::Data> = Vec::new();
        let mut contexts: std::collections::HashMap<String, Option<Vec<super::data::Data>>> =
            std::collections::HashMap::new();
        contexts.insert(DEFAULT_KEY.to_string(), None);
        for step in &self.steps {
            let (new_contexts, mut new_contents) = step
                .run_multiple(
                    contexts
                        .get(&step.load.clone().unwrap_or(DEFAULT_KEY.to_string()))
                        .unwrap()
                        .clone(),
                )
                .await;
            contents.append(&mut new_contents);
            let new_context = match new_contexts.is_empty() {
                true => None,
                false => Some(new_contexts),
            };
            if let Some(key) = &step.save_as {
                contexts.insert(key.clone(), new_context.clone());
            }
            contexts.insert(DEFAULT_KEY.to_string(), new_context);
        }
        contents.append(
            &mut contexts
                .get(DEFAULT_KEY)
                .unwrap()
                .clone()
                .unwrap_or_default(),
        );
        contents
    }

    pub async fn start(&self, pool: std::sync::Arc<crate::database::Connection>) {
        log::debug!(
            "starting watcher {} (triggered every {})...",
            &self.id,
            &self.every
        );
        // check the time duration
        let every = match humantime::parse_duration(&self.every) {
            Ok(every) => every,
            Err(e) => {
                log::error!("invalid 'every' field in template {}: {}", self.id, e);
                return;
            }
        };
        loop {
            // run the watcher
            log::debug!("running watcher {}", self.id);
            let results: Vec<super::data::Data> = self
                .run()
                .await
                .iter()
                .map(|data| {
                    let mut data = data.clone();
                    data.set_template(&self.id);
                    data
                })
                .collect();
            // save the results
            log::info!("{} result(s) found using watcher {}...", results.len(), self.id);
            if results.len() > 0 {
                for result in results {
                    match result.clone().into_model().save(&pool).await {
                        Ok(ev) => log::trace!(
                            "saved result {} from watcher {} ({}::{}B::{})",
                            ev.id,
                            ev.source,
                            ev.type_,
                            ev.data.len(),
                            ev.location
                        ),
                        Err(e) => log::warn!("error saving result for watcher {} ({}): {}", self.id, result.source, e),
                    }
                }
            }
            std::thread::sleep(every);
        }
    }
}

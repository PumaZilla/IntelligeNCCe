mod action;
mod data;
mod step;
mod trigger;
mod watcher;

use crate::error::{Error, Result};

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Templates {
    ids: std::collections::HashMap<String, bool>,
    watchers: Vec<watcher::TemplateWatcher>,
    triggers: std::collections::HashMap<data::DataType, Vec<trigger::TemplateTrigger>>,
}
impl Templates {
    pub fn new() -> Self {
        let mut triggers: std::collections::HashMap<data::DataType, Vec<trigger::TemplateTrigger>> =
            std::collections::HashMap::new();
        data::DataType::iter().for_each(|dt| {
            triggers.insert(dt.clone(), Vec::new());
        });
        Self {
            ids: std::collections::HashMap::new(),
            watchers: Vec::new(),
            triggers: triggers,
        }
    }

    pub fn add_watcher(&mut self, watcher: watcher::TemplateWatcher) -> Result<()> {
        if self.ids.contains_key(&watcher.id) {
            return Err(Error::TemplateDuplicatedError(watcher.id));
        }
        self.ids.insert(watcher.id.clone(), true);
        Ok(self.watchers.push(watcher))
    }

    pub fn add_trigger(&mut self, trigger: trigger::TemplateTrigger) -> Result<()> {
        if self.ids.contains_key(&trigger.id) {
            return Err(Error::TemplateDuplicatedError(trigger.id));
        }
        self.ids.insert(trigger.id.clone(), true);
        trigger
            .events
            .iter()
            .for_each(|dt| self.triggers.get_mut(dt).unwrap().push(trigger.clone()));
        Ok(())
    }

    pub async fn start(&self, pool: std::sync::Arc<crate::database::Connection>) {
        self.start_watchers(&pool).await;
    }

    async fn start_watchers(&self, pool: &std::sync::Arc<crate::database::Connection>) {
        log::debug!("starting {} watcher(s)...", self.watchers.len());
        // start all handlers and wait them to finish
        let mut handles = Vec::new();
        self.watchers.iter().for_each(|watcher| {
            let pool = pool.clone();
            let watcher = watcher.clone();
            handles.push(tokio::spawn(async move {
                watcher.start(pool).await;
            }));
        });
        futures::future::join_all(handles).await;
    }
}

// // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // //

/// The base structure of a generic template. Only field `type` is checked in order to determine the type of the template.
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TemplateBase {
    /// The template type.
    #[serde(rename = "type")]
    type_: TemplateTypes,
}
impl TemplateBase {
    pub fn from(template: &str) -> Result<Self> {
        Ok(serde_yaml::from_str(&template)
            .map_err(|e| Error::TemplateParseError(template.to_string(), e.to_string()))?)
    }
}

// // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // //

/// Template types. Two types are currently supported: `trigger` and `watcher`.
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
enum TemplateTypes {
    /// Trigger an action when data is received.
    Trigger,
    /// Monitor the state of a system periodically.
    Watcher,
}

// // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // //

/// Walks the given path and returns a list of all YML files.
fn find(root: &str) -> Result<Vec<String>> {
    let mut templates: Vec<String> = Vec::new();
    // read the given root
    for entry in std::fs::read_dir(root).map_err(|_| Error::IODirectoryError(root.to_string()))? {
        let path = entry
            .map_err(|_| Error::IOPathError(root.to_string()))?
            .path();
        // check ig the path is a directory
        if path.is_dir() {
            if let Some(newpath) = path.to_str() {
                templates.append(&mut find(newpath)?);
            }
        }
        // check if the path is a file with extension
        else if let Some(extension) = path.extension() {
            // check the extension
            if extension == "yml" || extension == "yaml" {
                // add the path to the list
                if let Some(file) = path.to_str() {
                    templates.push(file.to_string());
                }
            }
        }
    }
    // return all the YAML files
    Ok(templates)
}

/// Loads all templates from a given path.
pub fn load_all(path: &str) -> Result<Templates> {
    // find all templates
    let mut templates: Templates = Templates::new();
    find(path)?
        .iter()
        .map(|path| {
            // read the template
            let content =
                std::fs::read_to_string(path).map_err(|_| Error::IOReadError(path.to_string()))?;
            // check the base structure of a template
            let base = TemplateBase::from(&content)?;
            // convert the template
            match base.type_ {
                TemplateTypes::Watcher => {
                    templates.add_watcher(watcher::TemplateWatcher::from_template(&content)?)
                }
                TemplateTypes::Trigger => {
                    templates.add_trigger(trigger::TemplateTrigger::from_template(&content)?)
                }
            }?;
            Ok(())
        })
        .collect::<Vec<Result<()>>>()
        .into_iter()
        .filter(|res| res.is_err())
        .for_each(|res| {
            log::warn!("{}", res.err().unwrap());
        });
    Ok(templates)
}

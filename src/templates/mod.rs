mod action;
mod data;
mod step;
mod trigger;
mod watcher;

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

    pub fn add_watcher(
        &mut self,
        watcher: watcher::TemplateWatcher,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if self.ids.contains_key(&watcher.id) {
            return Err(format!(
                "cannot create watcher because template {} already exists",
                watcher.id
            )
            .into());
        }
        self.ids.insert(watcher.id.clone(), true);
        Ok(self.watchers.push(watcher))
    }

    pub fn add_trigger(
        &mut self,
        trigger: trigger::TemplateTrigger,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if self.ids.contains_key(&trigger.id) {
            return Err(format!(
                "cannot create trigger because template {} already exists",
                trigger.id
            )
            .into());
        }
        self.ids.insert(trigger.id.clone(), true);
        trigger
            .events
            .iter()
            .for_each(|dt| self.triggers.get_mut(dt).unwrap().push(trigger.clone()));
        Ok(())
    }

    pub fn get_triggers(
        &self,
        data_type: &data::DataType,
    ) -> Option<&Vec<trigger::TemplateTrigger>> {
        self.triggers.get(data_type)
    }
    pub fn get_watchers(&self) -> Vec<watcher::TemplateWatcher> {
        self.watchers.clone()
    }
}

// // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // //

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TemplateBase {
    #[serde(rename = "type")]
    type_: TemplateTypes,
}
impl TemplateBase {
    pub fn from(template: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(serde_yaml::from_str(&template)?)
    }
}

// // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // //

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
enum TemplateTypes {
    Watcher,
    Trigger,
}

// // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // //

/// Walks the given path and returns a list of all YML files
fn find(path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut templates: Vec<String> = Vec::new();
    // read the given path
    for entry in std::fs::read_dir(path)? {
        let path = entry?.path();
        // check ig the path is a directory
        if path.is_dir() {
            templates.append(&mut find(&path.to_str().ok_or("Unexpected path")?)?);
        }
        // check if the path is a file with extension
        else if let Some(extension) = path.extension() {
            // check the extension
            if extension == "yml" || extension == "yaml" {
                // add the path to the list
                if let Some(path) = path.to_str() {
                    templates.push(path.to_string());
                }
            }
        }
    }
    // return all the YAML files
    Ok(templates)
}

/// Loads all templates from a given path
pub fn load_all(path: &str) -> crate::error::Result<Templates> {
    // find all templates
    let mut templates: Templates = Templates::new();
    find(path)
        .map_err(|e| crate::error::Error::Unknown(e.to_string()))?
        .iter()
        .map(|path| {
            // read the template
            let content =
                std::fs::read_to_string(path)
                    .or(Err(format!("cannot read the file {}", path)).into())?;
            // check the base structure of a template
            let base = TemplateBase::from(&content).or_else(|err| {
                Err(format!(
                    "cannot get the template type from {}: {}",
                    path, err
                ))
                .into()
            })?;
            // convert the template
            match base.type_ {
                TemplateTypes::Watcher => templates.add_watcher(
                    watcher::TemplateWatcher::from_template(&content).or_else(|err| {
                        Err(format!("cannot create watcher from {}: {}", path, err)).into()
                    })?,
                ),
                TemplateTypes::Trigger => templates.add_trigger(
                    trigger::TemplateTrigger::from_template(&content).or_else(|err| {
                        Err(format!("cannot create watcher from {}: {}", path, err)).into()
                    })?,
                ),
            }?;
            Ok(())
        })
        .collect::<Vec<Result<(), Box<dyn std::error::Error>>>>()
        .into_iter()
        .filter(|res| res.is_err())
        .for_each(|res| {
            println!("[WARN] {}", res.err().unwrap());
        });
    Ok(templates)
}

mod action;
mod event;
mod step;

use crate::error::{Error, Result};

const DEFAULT_SLEEP_TICK: u64 = 2;
const DEFAULT_KEY: &str = " ::default";

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Templates {
    ids: std::collections::HashMap<String, bool>,
    templates: Vec<Template>,
}
impl Templates {
    pub fn new() -> Self {
        Self {
            ids: std::collections::HashMap::new(),
            templates: Vec::new(),
        }
    }

    fn len(&self) -> usize {
        self.ids.len()
    }

    pub fn append(&mut self, template: Template) -> Result<()> {
        if self.ids.contains_key(&template.id) {
            return Err(Error::TemplateDuplicatedError(template.id));
        }
        self.ids.insert(template.id.clone(), true);
        log::trace!("template \"{}\" loaded.", template.id);
        Ok(self.templates.push(template))
    }

    pub async fn run_all(&self, pool: std::sync::Arc<crate::database::DBConnection>) {
        log::debug!("starting {} template(s)...", self.len());
        // start all handlers and wait them to finish
        let mut handles = Vec::new();
        self.templates.iter().for_each(|template| {
            let pool = pool.clone();
            let template = template.clone();
            handles.push(tokio::spawn(async move {
                log::trace!("starting template thread for {}...", &template.id);
                template.start(pool).await;
                log::trace!("thread for {} finished.", &template.id);
            }));
        });
        futures::future::join_all(handles).await;
    }
}

// // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // //

/// The base structure of a generic template. Only field `type` is checked in order to determine the type of the template.
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Template {
    pub id: String,
    pub every: String,
    pub steps: Vec<step::TemplateStep>,
}
impl Template {
    pub fn from(template: &str) -> Result<Self> {
        Ok(serde_yaml::from_str(&template)
            .map_err(|e| Error::TemplateParseError(template.to_string(), e.to_string()))?)
    }

    pub async fn start(&self, pool: std::sync::Arc<crate::database::DBConnection>) {
        log::debug!(
            "starting template {} (triggered every {})...",
            &self.id,
            &self.every
        );
        // check the time duration
        let every = match humantime::parse_duration(&self.every) {
            Ok(every) => every,
            Err(e) => {
                log::error!("invalid 'every' attribute in template {}: {}", self.id, e);
                return;
            }
        };
        loop {
            // run the watcher
            log::debug!("executing template {}", self.id);
            let results: Vec<event::Event> = self
                .execute()
                .await
                .iter()
                .map(|data| {
                    let mut data = data.clone();
                    data.set_template(&self.id);
                    data
                })
                .collect();
            // check the results
            log::debug!(
                "{} result(s) found using template {}...",
                results.len(),
                self.id
            );
            if results.len() > 0 {
                // upload the keywords
                let mut keywords = std::collections::HashMap::<i32, regex::Regex>::new();
                let db_keywords = crate::database::models::keyword::Model::all(&pool).await;
                match db_keywords {
                    Ok(db_keywords) => {
                        db_keywords.iter().for_each(|keyword| {
                            match regex::RegexBuilder::new(&regex::escape(&keyword.value))
                                .case_insensitive(true)
                                .build()
                            {
                                Ok(re) => {
                                    keywords.insert(keyword.id, re);
                                }
                                Err(_) => {
                                    log::warn!("invalid keyword as regex ({})", keyword.value)
                                }
                            }
                        });
                    }
                    Err(e) => log::error!("error fetching keywords :: {}", e),
                };
                // check the raw results
                for result in results {
                    // check the keywords
                    if result.type_ == event::EventType::Raw {
                        let ids = result.check_content(&keywords);
                        if ids.len() > 0 {
                            // update the result
                            let model = result.into_model();
                            match model.save(&pool).await {
                                Ok(ev) => log::info!("new event found: {}", ev.source),
                                Err(e) => log::error!("error saving result :: {}", e),
                            };
                            // TODO: add event id + keyword id relationship
                            /*
                            ids.iter().for_each(|id| {
                                log::debug!(
                                    "Adding new event (keywords \"{}\" found at {})",
                                    keywords.get(id).unwrap(),
                                    result.source
                                );
                            });
                            */
                        }
                    }
                }
            }
            // wait for the next execution sleeping for 2 seconds each time
            let mut sleep = every.as_secs();
            while sleep > 0 {
                sleep -= DEFAULT_SLEEP_TICK;
                tokio::time::sleep(std::time::Duration::from_secs(DEFAULT_SLEEP_TICK)).await;
            }
        }
    }

    async fn execute(&self) -> Vec<event::Event> {
        let mut contents: Vec<event::Event> = Vec::new();
        let mut contexts: std::collections::HashMap<String, Option<Vec<event::Event>>> =
            std::collections::HashMap::new();
        contexts.insert(DEFAULT_KEY.to_string(), None);
        // run each step
        for step in &self.steps {
            // run the step with all the context and options
            let (new_contexts, mut new_contents) = step
                .run_multiple(
                    contexts
                        .get(&step.load.clone().unwrap_or(DEFAULT_KEY.to_string()))
                        .unwrap()
                        .clone(),
                )
                .await;
            // update the contents
            contents.append(&mut new_contents);
            // save the new contexts in the store
            let new_context = match new_contexts.is_empty() {
                true => None,
                false => Some(new_contexts),
            };
            if let Some(key) = &step.save_as {
                contexts.insert(key.clone(), new_context.clone());
            }
            contexts.insert(DEFAULT_KEY.to_string(), new_context);
        }
        // return the contents
        contents.append(
            &mut contexts
                .get(DEFAULT_KEY)
                .unwrap()
                .clone()
                .unwrap_or_default(),
        );
        contents
    }
}

// // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // //

/// Walks the given path and returns a list of all YML files.
fn find(root: &str) -> Result<Vec<String>> {
    let mut templates: Vec<String> = Vec::new();
    // read the given root
    log::trace!("reading directory \"{}\"...", root);
    for entry in std::fs::read_dir(root).map_err(|_| Error::IODirectoryError(root.to_string()))? {
        let path = entry
            .map_err(|_| Error::IOPathError(root.to_string()))?
            .path();
        // check ig the path is a directory
        log::trace!("checking if {:#?} is a directory...", path);
        if path.is_dir() {
            if let Some(newpath) = path.to_str() {
                templates.append(&mut find(newpath)?);
            }
        }
        // check if the path is a file with extension
        else if let Some(extension) = path.extension() {
            // check the extension
            log::trace!("checking if {:#?} is a YAML file...", path);
            if extension == "yml" || extension == "yaml" {
                // add the path to the list
                if let Some(file) = path.to_str() {
                    log::trace!("adding {} as a template.", file);
                    templates.push(file.to_string());
                }
            }
        }
    }
    // return all the YAML files
    Ok(templates)
}

/// Loads all templates from a given path.
pub fn load_all(disabled: bool, path: &str) -> Result<Templates> {
    let mut templates: Templates = Templates::new();
    // check if the templates should be disabled
    if disabled {
        log::warn!("templates are disabled.");
        return Ok(templates);
    }
    // find all templates
    find(path)?
        .iter()
        .map(|path| {
            // read the template
            log::trace!("reading template from {}", path);
            let content =
                std::fs::read_to_string(path).map_err(|_| Error::IOReadError(path.to_string()))?;
            // check the base structure of a template
            log::trace!("checking template structure for {}", path);
            Ok(Template::from(&content)?)
        })
        .for_each(|res: Result<Template>| match res {
            Ok(template) => match templates.append(template) {
                Err(e) => log::warn!("{}", e),
                _ => {}
            },
            Err(e) => log::warn!("{}", e),
        });
    log::info!("{} template(s) loaded.", templates.len());
    Ok(templates)
}

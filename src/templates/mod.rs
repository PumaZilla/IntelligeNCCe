mod action;
pub mod output;
mod step;
mod watch;

use crate::config;

/// Walks the given path and returns a list of all YML files
fn find(path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut templates: Vec<String> = Vec::new();
    for entry in std::fs::read_dir(path)? {
        let path = entry?.path();
        if let Some(extension) = path.extension() {
            if extension == "yml" || extension == "yaml" {
                if let Some(path) = path.to_str() {
                    templates.push(path.to_string());
                }
            }
        }
    }
    Ok(templates)
}

/// Loads all templates from a given path
pub fn load_all(
    path: &str,
) -> Result<std::collections::HashMap<String, Template>, Box<dyn std::error::Error>> {
    let mut templates: std::collections::HashMap<String, Template> =
        std::collections::HashMap::new();
    find(path)?.iter().for_each(|path| {
        match Template::new(path) {
            Ok(template) => {
                templates
                    .get(&template.id)
                    .ok_or_else(|| println!("[!] /Duplicate template ID: {}", template.id))
                    .ok();
                // TODO: Check if id is already in use
                if templates.get(&template.id).is_some() {
                    // TODO: Log warning
                    println!("[!] Duplicate template id: {}", template.id);
                } else {
                    templates.insert(template.id.clone(), template);
                }
            }
            Err(error) => {
                // TODO: Log error
                println!("[!] Failed to load template: {} :: {}", path, error);
            }
        }
    });
    Ok(templates)
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
/// Template represents a bunch of steps to be executed in order to obtain information
pub struct Template {
    /// Unique identifier for the template
    pub id: String,
    /// Watchers
    pub watch: Option<watch::Watch>,
}
impl Template {
    /// Creates a new template from the given path. The provided file must be a valid YAML file.
    pub fn new(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);
        let template: Self = serde_yaml::from_reader(reader)?;
        Ok(template)
    }
    /// Executes the template executing one by one each step and returns nothing
    pub async fn run(&self, cfg: &config::Config) {
        if let Some(workflow) = &self.watch {
            workflow.watch(cfg).await;
        }
    }
}

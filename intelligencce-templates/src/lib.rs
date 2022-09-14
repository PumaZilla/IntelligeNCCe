mod action;
mod output;
mod step;
mod watch;

use std::sync;

type Tx = sync::Arc<sync::Mutex<sync::mpsc::Sender<String>>>;

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
pub fn load_all(
    path: &str,
) -> Result<std::collections::HashMap<String, Template>, Box<dyn std::error::Error>> {
    let mut templates: std::collections::HashMap<String, Template> =
        std::collections::HashMap::new();
    find(path)?.iter().for_each(|path| {
        match Template::new(path) {
            Ok(template) => {
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

/// Connect to the repository and update all the templates
pub fn update(_repository: &str) -> Result<(), Box<dyn std::error::Error>> {
    unimplemented!();
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
/// Template represents a bunch of steps to be executed in order to obtain information
pub struct Template {
    /// Unique identifier for the template
    id: String,
    /// Watchers
    watch: Option<watch::Watch>,
}
impl Template {
    /// Creates a new template from the given path. The provided file must be a valid YAML file.
    pub fn new(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);
        let template: Self = serde_yaml::from_reader(reader)?;
        Ok(template)
    }
    /// Executes the watcher and send the outputs to the given channel
    pub fn watch(&self, tx: Tx) {
        if let Some(watch) = &self.watch {
            watch.start(tx);
        }
    }
}

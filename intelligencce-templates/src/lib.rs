const DEFAULT_KEY: &str = " ::default";

// // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // //

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Templates {
    ids: std::collections::HashMap<String, bool>,
    watchers: Vec<TemplateWatcher>,
    triggers: std::collections::HashMap<DataType, Vec<TemplateTrigger>>,
}
impl Templates {
    pub fn new() -> Self {
        let mut triggers: std::collections::HashMap<DataType, Vec<TemplateTrigger>> =
            std::collections::HashMap::new();
        DataType::iter().for_each(|dt| {
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
        watcher: TemplateWatcher,
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
        trigger: TemplateTrigger,
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

    pub fn get_triggers(&self, data_type: &DataType) -> Option<&Vec<TemplateTrigger>> {
        self.triggers.get(data_type)
    }
    pub fn get_watchers(&self) -> Vec<TemplateWatcher> {
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

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TemplateWatcher {
    id: String,
    every: String,
    steps: Vec<TemplateStep>,
}
impl TemplateWatcher {
    fn from(template: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(serde_yaml::from_str(&template)?)
    }

    fn run(&self) -> Vec<Data> {
        let mut contents: Vec<Data> = Vec::new();
        let mut contexts: std::collections::HashMap<String, Option<Vec<Data>>> =
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

    pub fn start(&self, tx: std::sync::Arc<std::sync::Mutex<std::sync::mpsc::Sender<Data>>>) {
        println!(
            "[*] Starting watcher {} (triggered every {})...",
            &self.id, &self.every
        );
        match humantime::parse_duration(&self.every) {
            Ok(duration) => {
                loop {
                    println!("[*] Running watcher {}...", self.id);
                    let results: Vec<Data> = self
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

// // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // //

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TemplateTrigger {
    id: String,
    events: Vec<DataType>,
    #[serde(rename = "steps")]
    _steps: Vec<TemplateStep>,
}
impl TemplateTrigger {
    fn from(template: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(serde_yaml::from_str(&template)?)
    }
}

// // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // //

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
struct TemplateStep {
    name: Option<String>,
    action: TemplateAction,
    load: Option<String>,
    save_as: Option<String>,
    options: Option<std::collections::HashMap<String, String>>,
}
impl std::fmt::Display for TemplateStep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.name.clone().unwrap_or(self.action.to_string())
        )
    }
}
impl TemplateStep {
    fn run(&self, context: Option<Data>) -> (Vec<Data>, Vec<Data>) {
        println!("[*] {}", self);
        let ctx = context.clone();
        let opts = self.replace_variables(ctx);
        self.action.run(context, opts)
    }

    fn run_multiple(&self, context: Option<Vec<Data>>) -> (Vec<Data>, Vec<Data>) {
        match context {
            Some(context) => context.iter().fold(
                (Vec::new(), Vec::new()),
                |(mut context, mut content), ctx| {
                    let (mut ctx, mut ctn) = self.run(Some(ctx.clone()));
                    context.append(&mut ctx);
                    content.append(&mut ctn);
                    (context, content)
                },
            ),
            None => self.run(None),
        }
    }

    fn replace_variables(
        &self,
        context: Option<Data>,
    ) -> Option<std::collections::HashMap<String, String>> {
        let re = regex::Regex::new(r"\{\s*(\$?[a-zA-Z0-9_]+)\s*\}").unwrap();
        self.options.clone().and_then(|options| {
            let mut new_options = std::collections::HashMap::new();
            options.iter().for_each(|(key, value)| {
                let new_value: String = match context.clone() {
                    Some(context) => {
                        let new_value = re.replace_all(value, |caps: &regex::Captures| {
                            if caps.len() > 1 {
                                let placeholder = &caps[1];
                                if placeholder.starts_with("$") {
                                    std::env::var(&placeholder[1..]).unwrap_or_default()
                                } else {
                                    match placeholder {
                                        "source" => &context.source,
                                        "content" => &context.content,
                                        _ => placeholder,
                                    }
                                    .to_string()
                                }
                            } else {
                                caps[0].to_string()
                            }
                        });
                        new_value.to_string()
                    }
                    None => value.clone(),
                };
                new_options.insert(key.clone(), new_value);
            });
            Some(new_options)
        })
    }
}

// // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // //

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TemplateAction {
    Debug,
    Extract,
    Fetch,
}
impl std::fmt::Display for TemplateAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Debug => "Debugging the information",
                Self::Extract => "Extracting data from previous step",
                Self::Fetch => "Sending a request",
            }
        )
    }
}
impl TemplateAction {
    fn run(
        &self,
        context: Option<Data>,
        options: Option<std::collections::HashMap<String, String>>,
    ) -> (Vec<Data>, Vec<Data>) {
        match self.execute(context, options) {
            Ok((context, content)) => (context, content),
            Err(err) => {
                println!("Error {}: {}", self, err);
                (Vec::new(), Vec::new())
            }
        }
    }

    fn execute(
        &self,
        context: Option<Data>,
        options: Option<std::collections::HashMap<String, String>>,
    ) -> Result<(Vec<Data>, Vec<Data>), Box<dyn std::error::Error>> {
        Ok(match self {
            Self::Debug => {
                let ctx = context.ok_or("missing context")?;
                println!(
                    "{}",
                    options
                        .unwrap_or_default()
                        .get("message")
                        .unwrap_or(&ctx.content)
                );
                (vec![ctx], Vec::new())
            }
            Self::Extract => {
                // check the values
                let options = options.ok_or("missing options")?;
                let context = context.ok_or("missing context")?;
                let query = options.get("query").ok_or("missing query")?;
                let group: usize = options
                    .get("group")
                    .unwrap_or(&"0".to_string())
                    .parse()
                    .map_err(|err| format!("cannot parse group as integer: {}", err))?;
                let re = regex::Regex::new(query)
                    .or_else(|err| Err(format!("invalid query {}: {}", query, err)))?;
                let mut content = Vec::new();
                re.captures_iter(&context.content).for_each(|capture| {
                    if capture.len() > group {
                        content.push(Data::content_from(context.clone(), &capture[group]));
                    }
                });
                (content, Vec::new())
            }
            Self::Fetch => {
                // check the values
                let options: std::collections::HashMap<String, String> = match options {
                    Some(options) => options,
                    None => Some(context.clone().ok_or("options missing")?)
                        .and(Some(options.unwrap_or_default()))
                        .unwrap(),
                };
                let url: String = match options.get("url") {
                    Some(url) => url.to_string(),
                    None => reqwest::Url::parse(
                        &context
                            .clone()
                            .ok_or("there is not a valid context")?
                            .content,
                    )
                    .map_err(|err| format!("cannot parse URL from previous step: {}", err))?
                    .to_string(),
                };
                let method: reqwest::Method = reqwest::Method::from_bytes(
                    options
                        .get("method")
                        .unwrap_or(&"GET".to_string())
                        .as_bytes(),
                )?;
                // parse the headers
                let headers: std::collections::HashMap<String, String> = options
                    .get("headers")
                    .unwrap_or(&"User-Agent: intelligennce".to_string())
                    .split("\n")
                    // step: no empty lines
                    .map(|line| line.trim())
                    .filter(|line| !line.is_empty())
                    // step: split the header name and value
                    .map(|line| line.splitn(2, ":").map(|s| s.trim()).collect::<Vec<&str>>())
                    .filter(|parts| parts.len() == 2)
                    // step: create the headers
                    .map(|header| (header[0].to_string(), header[1].to_string()))
                    .collect();
                // create the request
                let res = reqwest::blocking::Client::new()
                    .request(method, &url)
                    .headers((&headers).try_into()?)
                    .send()?
                    .text()?;
                // send the request
                (
                    vec![Data::from(context.clone().unwrap_or_default(), &url, &res)],
                    Vec::new(),
                )
            }
        })
    }
}

// // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // //

#[derive(Debug, Clone)]
pub struct Data {
    pub template: String,
    pub source: String,
    pub content: String,
}
impl Default for Data {
    fn default() -> Self {
        Self {
            template: "-- Unknown template".to_string(),
            source: "-- Unknown source".to_string(),
            content: "-- No content".to_string(),
        }
    }
}
impl Data {
    pub fn new(template: &str, source: &str, content: &str) -> Self {
        Self {
            template: template.to_string(),
            source: source.to_string(),
            content: content.to_string(),
        }
    }
    pub fn from(prev: Self, source: &str, content: &str) -> Self {
        Self {
            template: prev.template,
            source: source.to_string(),
            content: content.to_string(),
        }
    }
    pub fn content_from(prev: Self, content: &str) -> Self {
        Self {
            template: prev.template,
            source: prev.source,
            content: content.to_string(),
        }
    }
    pub fn set_template(&mut self, template: &str) {
        self.template = template.to_string();
    }
}

// // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // //

#[derive(Debug, Eq, PartialEq, Hash, Clone, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DataType {
    Source,
    Email,
}
impl DataType {
    fn iter() -> std::slice::Iter<'static, Self> {
        static TYPES: [DataType; 2] = [DataType::Source, DataType::Email];
        TYPES.iter()
    }
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
pub fn load_all(path: &str) -> Result<Templates, Box<dyn std::error::Error>> {
    // find all templates
    let mut templates: Templates = Templates::new();
    find(path)?
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
                TemplateTypes::Watcher => {
                    templates.add_watcher(TemplateWatcher::from(&content).or_else(|err| {
                        Err(format!("cannot create watcher from {}: {}", path, err)).into()
                    })?)
                }
                TemplateTypes::Trigger => {
                    templates.add_trigger(TemplateTrigger::from(&content).or_else(|err| {
                        Err(format!("cannot create watcher from {}: {}", path, err)).into()
                    })?)
                }
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

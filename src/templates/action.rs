use super::output;
use crate::config;

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Action {
    Fetch,
    Debug,
    Extract,
    Template,
    Publish,
}

impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fetch => write!(f, "request"),
            Self::Debug => write!(f, "log"),
            Self::Extract => write!(f, "extract"),
            Self::Template => write!(f, "template"),
            Self::Publish => write!(f, "publish"),
        }
    }
}

impl Action {
    pub async fn execute(
        &self,
        cfg: &config::Config,
        arg: &output::Output,
        options: &std::collections::HashMap<String, String>,
    ) -> Result<Vec<output::Output>, Box<dyn std::error::Error>> {
        Ok(match self {
            Self::Publish => {
                cfg.storage
                    .publish(options.get("type").unwrap_or(&"source".to_string()), arg)
                    .await?; // FIXME: Create default type
                vec![]
            }
            Self::Debug => {
                let message = options
                    .get("message")
                    .unwrap_or(&"{data}".to_string())
                    .replace("{data}", &arg.data)
                    .replace("{source}", &arg.src);
                println!("{}", message);
                vec![arg.clone()]
            }
            Self::Template => {
                let template = options
                    .get("template")
                    .ok_or("No template provided")?
                    .replace("{source}", &arg.src)
                    .replace("{data}", &arg.data);
                vec![output::Output {
                    src: arg.src.clone(),
                    data: template,
                }]
            }
            Self::Extract => {
                let default_type: &std::string::String = &"regex".to_string();
                let query: &str = options.get("query").ok_or("No query provided")?;
                match options.get("type").unwrap_or(default_type).as_ref() {
                    "text" | "regex" | "regexp" => {
                        let re = regex::Regex::new(query)?;
                        let group: usize =
                            options.get("group").unwrap_or(&"0".to_string()).parse()?;
                        let mut res: Vec<output::Output> = Vec::new();
                        for cap in re.captures_iter(&arg.data) {
                            res.push(output::Output::new(
                                arg.src.clone(),
                                cap.get(group)
                                    .ok_or("Group not specified")?
                                    .as_str()
                                    .to_string(),
                            ));
                        }
                        res
                    }
                    "json" => unimplemented!(),
                    "html" | "xpath" => unimplemented!(),
                    _ => Err("Invalid filter type")?,
                }
            }
            Self::Fetch => {
                // Check the values
                let url: String = match options.get("url") {
                    Some(url) => url.to_string(),
                    None => reqwest::Url::parse(&arg.data)?.as_str().to_string(),
                };
                let method: reqwest::Method = reqwest::Method::from_bytes(
                    options
                        .get("method")
                        .unwrap_or(&"GET".to_string())
                        .as_bytes(),
                )?;
                // Parse the headers
                let headers: std::collections::HashMap<String, String> = options
                    .get("headers")
                    .unwrap_or(&"User-Agent: intelligennce".to_string())
                    .split("\n")
                    // no empty lines
                    .map(|line| line.trim())
                    .filter(|line| !line.is_empty())
                    // split the header name and value
                    .map(|line| line.splitn(2, ":").map(|s| s.trim()).collect::<Vec<&str>>())
                    .filter(|parts| parts.len() == 2)
                    // create the headers
                    .map(|header| (header[0].to_string(), header[1].to_string()))
                    .collect();
                // Create the request
                let res = reqwest::Client::new()
                    .request(method, &url)
                    .headers((&headers).try_into()?)
                    .send()
                    .await?
                    .text()
                    .await?;
                // Send the request
                vec![output::Output::new(url.clone(), res)]
            }
        })
    }
}

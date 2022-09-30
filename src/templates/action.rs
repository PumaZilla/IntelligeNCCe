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
    pub fn run(
        &self,
        context: Option<super::data::Data>,
        options: Option<std::collections::HashMap<String, String>>,
    ) -> (Vec<super::data::Data>, Vec<super::data::Data>) {
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
        context: Option<super::data::Data>,
        options: Option<std::collections::HashMap<String, String>>,
    ) -> Result<(Vec<super::data::Data>, Vec<super::data::Data>), Box<dyn std::error::Error>> {
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
                        content.push(super::data::Data::content_from(
                            context.clone(),
                            &capture[group],
                        ));
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
                    vec![super::data::Data::from(
                        context.clone().unwrap_or_default(),
                        &url,
                        &res,
                    )],
                    Vec::new(),
                )
            }
        })
    }
}

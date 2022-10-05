use crate::error::{Error, Result};

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
                Self::Debug => "debugging the information",
                Self::Extract => "extracting data from previous step",
                Self::Fetch => "sending a request",
            }
        )
    }
}
impl TemplateAction {
    pub async fn run(
        &self,
        context: Option<super::data::Data>,
        options: Option<std::collections::HashMap<String, String>>,
    ) -> (Vec<super::data::Data>, Vec<super::data::Data>) {
        match self.execute(context, options).await {
            Ok((context, content)) => (context, content),
            Err(err) => {
                println!("Error {}: {}", self, err);
                (Vec::new(), Vec::new())
            }
        }
    }

    async fn execute(
        &self,
        context: Option<super::data::Data>,
        options: Option<std::collections::HashMap<String, String>>,
    ) -> Result<(Vec<super::data::Data>, Vec<super::data::Data>)> {
        Ok(match self {
            Self::Debug => {
                let ctx = context.ok_or(Error::TemplateActionNoContextError(self.to_string()))?;
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
                let options =
                    options.ok_or(Error::TemplateActionNoOptionsError(self.to_string()))?;
                let context =
                    context.ok_or(Error::TemplateActionNoContextError(self.to_string()))?;
                let query = options
                    .get("query")
                    .ok_or(Error::TemplateActionNoOptionError(
                        self.to_string(),
                        "query".to_string(),
                    ))?;
                let group: usize = options
                    .get("group")
                    .unwrap_or(&"0".to_string())
                    .parse()
                    .map_err(|e| {
                        Error::TemplateActionExecError(
                            self.to_string(),
                            format!("cannot parse group as integer ({})", e),
                        )
                    })?;
                let re = regex::Regex::new(query).map_err(|e| {
                    Error::TemplateActionExecError(
                        self.to_string(),
                        format!("invalid query ({})", e),
                    )
                })?;
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
                    None => Some(
                        context
                            .clone()
                            .ok_or(Error::TemplateActionNoOptionsError(self.to_string()))?,
                    )
                    .and(Some(options.unwrap_or_default()))
                    .unwrap(),
                };
                let url: String = match options.get("url") {
                    Some(url) => url.to_string(),
                    None => reqwest::Url::parse(
                        &context
                            .clone()
                            .ok_or(Error::TemplateActionNoContextError(self.to_string()))?
                            .content,
                    )
                    .map_err(|e| {
                        Error::TemplateActionExecError(
                            self.to_string(),
                            format!("cannot parse URL from previous step ({})", e),
                        )
                    })?
                    .to_string(),
                };
                let method: reqwest::Method = reqwest::Method::from_bytes(
                    options
                        .get("method")
                        .unwrap_or(&"GET".to_string())
                        .as_bytes(),
                )
                .map_err(|_| {
                    Error::TemplateActionNoOptionError(self.to_string(), "method".to_string())
                })?;
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
                let res = reqwest::Client::new()
                    .request(method, &url)
                    .headers((&headers).try_into().map_err(|e| {
                        Error::TemplateActionExecError(
                            self.to_string(),
                            format!("unable to parse the headers ({})", e),
                        )
                    })?)
                    .send().await
                    .map_err(|e| {
                        Error::TemplateActionExecError(
                            self.to_string(),
                            format!("unable to send the rquest ({})", e),
                        )
                    })?
                    .text().await
                    .map_err(|e| {
                        Error::TemplateActionExecError(
                            self.to_string(),
                            format!("unable to retrieve the body ({})", e),
                        )
                    })?;

                // return the request
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

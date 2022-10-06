#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TemplateStep {
    pub name: Option<String>,
    pub action: super::action::TemplateAction,
    pub load: Option<String>,
    pub save_as: Option<String>,
    pub options: Option<std::collections::HashMap<String, String>>,
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
    pub async fn run(
        &self,
        context: Option<super::event::Event>,
    ) -> (Vec<super::event::Event>, Vec<super::event::Event>) {
        let ctx = context.clone();
        let opts = self.replace_variables(ctx);
        log::debug!("{}", self);
        self.action.run(context, opts).await
    }

    pub async fn run_multiple(
        &self,
        context: Option<Vec<super::event::Event>>,
    ) -> (Vec<super::event::Event>, Vec<super::event::Event>) {
        match context {
            Some(context) => {
                let mut new_context = Vec::new();
                let mut new_content = Vec::new();
                for ctx in context {
                    let (ctx, content) = self.run(Some(ctx)).await;
                    new_context.extend(ctx);
                    new_content.extend(content);
                }
                (new_context, new_content)
            },
            None => self.run(None).await,
        }
    }

    fn replace_variables(
        &self,
        context: Option<super::event::Event>,
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
                                        "content" => &context.data,
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

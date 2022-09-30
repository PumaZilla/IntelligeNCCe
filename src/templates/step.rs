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
    pub fn run(
        &self,
        context: Option<super::data::Data>,
    ) -> (Vec<super::data::Data>, Vec<super::data::Data>) {
        println!("[*] {}", self);
        let ctx = context.clone();
        let opts = self.replace_variables(ctx);
        self.action.run(context, opts)
    }

    pub fn run_multiple(
        &self,
        context: Option<Vec<super::data::Data>>,
    ) -> (Vec<super::data::Data>, Vec<super::data::Data>) {
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
        context: Option<super::data::Data>,
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

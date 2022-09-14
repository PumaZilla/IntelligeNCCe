use super::action;
//use super::output;

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Step {
    name: Option<String>,
    action: action::Action,
    _load: Option<String>,
    _save_as: Option<String>,
    _options: Option<std::collections::HashMap<String, String>>,
}
impl std::fmt::Display for Step {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            &self.name.clone().unwrap_or(self.action.to_string())
        )
    }
}
/*
impl Step {
    pub async fn run(
        &self,
        cfg: &config::Config,
        args: &Vec<output::Output>,
    ) -> Result<Vec<output::Output>, Box<dyn std::error::Error>> {
        println!("[*] Running step: {}", self);
        let mut res: Vec<output::Output> = Vec::new();
        let options = &self.options.clone().unwrap_or_default();
        match args.len() {
            0 => {
                res = self
                    .action
                    .execute(cfg, &output::Output::default(), options)
                    .await?;
            }
            _ => {
                for arg in args {
                    res.append(&mut self.action.execute(cfg, arg, options).await?)
                }
            }
        }
        Ok(res)
    }
}
*/

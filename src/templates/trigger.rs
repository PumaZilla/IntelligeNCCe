use crate::error::{Error, Result};

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TemplateTrigger {
    pub id: String,
    pub events: Vec<super::data::DataType>,
    #[serde(rename = "steps")]
    pub _steps: Vec<super::step::TemplateStep>,
}
impl TemplateTrigger {
    pub fn from_template(template: &str) -> Result<Self> {
        Ok(serde_yaml::from_str(&template).map_err(|e| Error::TemplateParseError(template.to_string(), e.to_string()))?)
    }
}

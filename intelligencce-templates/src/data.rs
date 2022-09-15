#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Data {
    pub template: String,
    pub type_: String,
    pub src: String,
    pub content: String,
}
impl Default for Data {
    fn default() -> Self {
        Self {
            template: "-- No template --".to_string(),
            type_: "source".to_string(),
            src: "-- No source --".to_string(),
            content: String::new(),
        }
    }
}
impl Data {
    pub fn new(template: &str, type_: &str, src: &str, content: &str) -> Self {
        Self {
            template: template.to_string(),
            type_: type_.to_string(),
            src: src.to_string(),
            content: content.to_string(),
        }
    }
    pub fn deserialize(&self, content: String) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(serde_json::from_str(&content)?)
    }
}

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
    pub fn iter() -> std::slice::Iter<'static, Self> {
        static TYPES: [DataType; 2] = [DataType::Source, DataType::Email];
        TYPES.iter()
    }
}

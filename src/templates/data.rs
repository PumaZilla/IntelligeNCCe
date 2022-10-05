#[derive(Debug, Clone)]
pub struct Data {
    pub template: String,
    pub type_: DataType,
    pub source: String,
    pub content: String,
}
impl Default for Data {
    fn default() -> Self {
        Self {
            template: "-- Unknown template".to_string(),
            type_: DataType::Source,
            source: "-- Unknown source".to_string(),
            content: "-- No content".to_string(),
        }
    }
}
impl Data {
    pub fn new(template: &str, type_: DataType, source: &str, content: &str) -> Self {
        Self {
            type_,
            template: template.to_string(),
            source: source.to_string(),
            content: content.to_string(),
        }
    }
    pub fn from(prev: Self, source: &str, content: &str) -> Self {
        Self {
            template: prev.template,
            type_: prev.type_,
            source: source.to_string(),
            content: content.to_string(),
        }
    }
    pub fn content_from(prev: Self, content: &str) -> Self {
        Self {
            template: prev.template,
            type_: prev.type_,
            source: prev.source,
            content: content.to_string(),
        }
    }
    pub fn set_template(&mut self, template: &str) {
        self.template = template.to_string();
    }

    pub fn into_model(self) -> crate::database::models::event::NewModel {
        crate::database::models::event::NewModel {
            source: self.template,
            type_: self.type_.to_string(),
            location: self.source,
            data: self.content,
        }
    }
}

// // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // //

#[derive(Debug, Eq, PartialEq, Hash, Clone, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DataType {
    Domain,
    Email,
    Source,
}
impl DataType {
    pub fn iter() -> std::slice::Iter<'static, Self> {
        static TYPES: [DataType; 3] = [DataType::Domain,DataType::Email,DataType::Source];
        TYPES.iter()
    }
}
impl std::fmt::Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataType::Domain => write!(f, "domain"),
            DataType::Email => write!(f, "email"),
            DataType::Source => write!(f, "source"),
        }
    }
}

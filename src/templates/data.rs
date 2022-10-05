#[derive(Debug, Clone)]
pub struct Data {
    pub template: String,
    pub type_: DataType,
    pub source: String,
    pub data: String,
}
impl Default for Data {
    fn default() -> Self {
        Self {
            template: "-- Unknown template".to_string(),
            type_: DataType::Source,
            source: "-- Unknown source".to_string(),
            data: "-- No data".to_string(),
        }
    }
}
impl Data {
    pub fn from(prev: Self, source: &str, data: &str) -> Self {
        log::trace!("updating data from {} (prev: {})", source, prev.source);
        Self {
            template: prev.template,
            type_: prev.type_,
            source: source.to_string(),
            data: data.to_string(),
        }
    }
    pub fn data_from(prev: Self, data: &str) -> Self {
        log::trace!("updating data from {}", prev.source);
        Self {
            template: prev.template,
            type_: prev.type_,
            source: prev.source,
            data: data.to_string(),
        }
    }
    pub fn set_template(&mut self, template: &str) {
        self.template = template.to_string();
    }

    pub fn into_model(self) -> crate::database::models::event::NewModel {
        log::trace!("converting data into model...");
        crate::database::models::event::NewModel {
            template: self.template,
            type_: self.type_.to_string(),
            source: self.source,
            data: self.data,
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
    pub fn _iter() -> std::slice::Iter<'static, Self> {
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

#[derive(Debug, Clone)]
pub struct Event {
    pub template: String,
    pub type_: EventType,
    pub source: String,
    pub data: String,
}
impl Default for Event {
    fn default() -> Self {
        Self {
            template: "-- Unknown template".to_string(),
            type_: EventType::Raw,
            source: "-- Unknown source".to_string(),
            data: "-- No data".to_string(),
        }
    }
}
impl Event {
    pub fn from(prev: Self, source: &str, data: &str) -> Self {
        log::trace!("updating event from {} (prev: {})", source, prev.source);
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
        log::trace!("converting event into model...");
        crate::database::models::event::NewModel {
            template: self.template,
            type_: self.type_.to_string(),
            source: self.source,
            data: self.data,
        }
    }

    pub fn check_content(
        &self,
        keywords: &std::collections::HashMap<i32, regex::Regex>,
    ) -> Vec<i32> {
        keywords
            .iter()
            .filter_map(|(id, re)| {
                if re.is_match(&self.data) {
                    Some(*id)
                } else {
                    None
                }
            })
            .collect()
    }
}

// // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // //

#[derive(Debug, Eq, PartialEq, Hash, Clone, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    Domain,
    Email,
    Raw,
}
impl EventType {
    pub fn _iter() -> std::slice::Iter<'static, Self> {
        static TYPES: [EventType; 3] = [EventType::Domain, EventType::Email, EventType::Raw];
        TYPES.iter()
    }
}
impl std::fmt::Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventType::Domain => write!(f, "domain"),
            EventType::Email => write!(f, "email"),
            EventType::Raw => write!(f, "raw"),
        }
    }
}

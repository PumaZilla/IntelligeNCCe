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
            type_: EventType::Unknown,
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

    pub fn into_model(self) -> crate::database::models::NewEvent {
        log::trace!("converting event into model...");
        // use crate::database::models::EventType;
        crate::database::models::NewEvent {
            template: self.template,
            type_: self.type_.to_string().into(),
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
#[serde(rename_all = "lowercase")]
pub enum EventType {
    Unknown,
    Paste,
    Blacklist,

    IP,
    Domain,
    URL,
    Email,
}
impl EventType {
    // FIXME: This is used for something?
    pub fn _iter() -> std::slice::Iter<'static, Self> {
        static TYPES: [EventType; 7] = [
            EventType::Unknown,
            EventType::Paste,
            EventType::Blacklist,
            EventType::IP,
            EventType::Domain,
            EventType::URL,
            EventType::Email,
        ];
        TYPES.iter()
    }
}
impl std::fmt::Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "raw"),
            Self::Paste => write!(f, "paste"),
            Self::Blacklist => write!(f, "blacklist"),
            Self::IP => write!(f, "ip"),
            Self::Domain => write!(f, "domain"),
            Self::URL => write!(f, "url"),
            Self::Email => write!(f, "email"),
        }
    }
}

/*
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Output {
    pub src: String,
    pub data: String,
}
impl Default for Output {
    fn default() -> Self {
        Self {
            src: "-- No source --".to_string(),
            data: String::new(),
        }
    }
}
impl Output {
    pub fn new(src: String, data: String) -> Self {
        Self { src, data }
    }
    pub fn serialize(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(serde_json::to_string(&self)?)
    }
    pub fn deserialize(&self, data: String) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(serde_json::from_str(&data)?)
    }
}
*/
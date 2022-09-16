pub struct Storage {
    pub cache: redis::Client,
}
impl Storage {
    pub fn new(cfg: &super::config::Config) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            cache: redis::Client::open(cfg.redis.clone())?,
        })
    }

    pub fn publish(&self, data: &intelligencce_templates::Data) -> Result<(), Box<dyn std::error::Error>> {
        let mut con = self.cache.get_connection()?;
        redis::cmd("PUBLISH").arg(&data.type_).arg(data.serialize()?).query(&mut con)?;
        Ok(())
    }
}
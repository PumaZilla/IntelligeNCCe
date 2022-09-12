use crate::templates::output;

#[derive(Clone)]
pub struct Config {
    pub storage: Storage,
}
impl Config {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            storage: Storage::new()?,
        })
    }
}

#[derive(Clone)]
pub struct Storage {
    pub cache: redis::Client,
}
impl Storage {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            cache: redis::Client::open("redis://127.0.0.1/")?,
        })
    }
    pub async fn publish(
        &self,
        key: &str,
        output: &output::Output,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut conn = self.cache.get_async_connection().await?;
        redis::cmd("PUBLISH")
            .arg(key)
            .arg(output.serialize()?)
            .query_async(&mut conn)
            .await?;
        Ok(())
    }
}

use reqwest::Client;
use serde_json::Value;

#[derive(Clone)]
pub struct AgTechClient {
    client: Client,
    base_url: String,
}

impl AgTechClient {
    pub fn new(base_url: String) -> Self {
        Self { client: Client::new(), base_url }
    }

    pub async fn get(&self, path: &str) -> anyhow::Result<Value> {
        Ok(self.client.get(format!("{}{}", self.base_url, path)).send().await?.json().await?)
    }

    pub async fn get_query(&self, path: &str, params: &[(&str, &str)]) -> anyhow::Result<Value> {
        Ok(self.client.get(format!("{}{}", self.base_url, path)).query(params).send().await?.json().await?)
    }

    pub async fn post(&self, path: &str, body: &Value) -> anyhow::Result<Value> {
        Ok(self.client.post(format!("{}{}", self.base_url, path)).json(body).send().await?.json().await?)
    }

    pub async fn patch(&self, path: &str, body: &Value) -> anyhow::Result<Value> {
        Ok(self.client.patch(format!("{}{}", self.base_url, path)).json(body).send().await?.json().await?)
    }

    pub async fn delete(&self, path: &str) -> anyhow::Result<Value> {
        Ok(self.client.delete(format!("{}{}", self.base_url, path)).send().await?.json().await?)
    }
}

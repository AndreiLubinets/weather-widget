use anyhow::{Error, Result};
use async_trait::async_trait;
use druid::ImageBuf;
use reqwest::IntoUrl;

#[async_trait]
pub trait FromUrl {
    async fn from_url<T: IntoUrl + Send>(url: T) -> Result<Self>
    where
        Self: std::marker::Sized;
}

#[async_trait]
impl FromUrl for ImageBuf {
    async fn from_url<T: IntoUrl + Send>(url: T) -> Result<Self> {
        let bytes = reqwest::get(url).await?.bytes().await?;
        ImageBuf::from_data(&bytes).map_err(|err| Error::msg(err.to_string()))
    }
}

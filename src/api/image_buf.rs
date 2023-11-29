use anyhow::{Error, Result};
use druid::ImageBuf;
use futures::TryFutureExt;
use reqwest::IntoUrl;

pub trait FromUrl {
    fn from_url(url: impl IntoUrl) -> Result<Self>
    where
        Self: std::marker::Sized;
}

impl FromUrl for ImageBuf {
    fn from_url(url: impl IntoUrl) -> Result<Self> {
        //TODO: remove blocking
        let bytes = futures::executor::block_on(reqwest::get(url).and_then(|res| res.bytes()))?;
        ImageBuf::from_data(&bytes).map_err(|err| Error::msg(err.to_string()))
    }
}

use anyhow::{Error, Result};
use druid::ImageBuf;
use reqwest::{blocking, IntoUrl};

pub trait FromUrl {
    fn from_url(url: impl IntoUrl) -> Result<Self>
    where
        Self: std::marker::Sized;
}

impl FromUrl for ImageBuf {
    fn from_url(url: impl IntoUrl) -> Result<Self> {
        let bytes = blocking::get(url)?.bytes()?;
        ImageBuf::from_data(&bytes).map_err(|err| Error::msg(err.to_string()))
    }
}

// Copyright (c) 2022 MobileCoin Foundation

use crate::{ConfigError, Location, LocationProvider};
use reqwest::{
  blocking::Client,
  header::{HeaderMap, HeaderValue, CONTENT_TYPE},
};

pub struct IpInfoIoFetch;

impl LocationProvider for IpInfoIoFetch {
  fn location(&self) -> Result<Location, ConfigError> {
    let mut json_headers = HeaderMap::new();
    json_headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    let client = Client::builder()
      .default_headers(json_headers)
      .gzip(true)
      .use_rustls_tls()
      .build()?;
    let response = client
      .get("https://ipinfo.io/json/")
      .send()?
      .error_for_status()?;
    Ok(response.json()?)
  }
}

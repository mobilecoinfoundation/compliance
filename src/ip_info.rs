// Copyright (c) 2022 MobileCoin Foundation

use crate::{Configuration, Error, Location, LocationProvider};
use reqwest::{
  blocking::Client,
  header::{HeaderMap, HeaderValue, CONTENT_TYPE},
};

pub struct IpInfoIoFetch;

impl LocationProvider for IpInfoIoFetch {
  fn location(&self, config: &Option<Configuration>) -> Result<Location, Error> {
    let mut json_headers = HeaderMap::new();
    json_headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    let client = Client::builder()
      .default_headers(json_headers)
      .gzip(true)
      .use_rustls_tls()
      .build()?;

    let mut url = "https://ipinfo.io/json/".to_string();
    match config {
      Some(config) => match config.ip_info_key.borrow() {
        Some(ip_info_key) => url = url + "?token" + ip_info_key.as_str(),
        _ => {}
      },
      _ => {}
    }
    let response = client
      .get(url)
      .send()?
      .error_for_status()?;
    Ok(response.json()?)
  }
}

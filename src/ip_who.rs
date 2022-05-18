// Copyright (c) 2022 MobileCoin Foundation

use crate::{Error, Location, LocationProvider};
use reqwest::{
  blocking::Client,
  header::{HeaderMap, HeaderValue, CONTENT_TYPE},
};

// Fetch from ipwho.is
pub struct IpWhoIs;

impl LocationProvider for IpWhoIs {
  fn location(&self) -> Result<Location, Error> {
    let mut json_headers = HeaderMap::new();
    json_headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    let client = Client::builder()
      .default_headers(json_headers)
      .gzip(true)
      .use_rustls_tls()
      .build()?;
    let response = client
      .get("https://ipwho.is/")
      .send()?
      .error_for_status()?;
    Ok(response.json()?)
  }
}

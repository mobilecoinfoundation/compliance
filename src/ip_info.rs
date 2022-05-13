use crate::{ConfigError, Location, LocationProvider};
use reqwest::{
  blocking::Client,
  header::{HeaderMap, HeaderValue, CONTENT_TYPE},
};

pub struct IpInfoIoFetch;

impl LocationProvider for IpInfoIoFetch {
  fn location(&self) -> Result<Location, ConfigError> {
    let client = Client::builder().gzip(true).use_rustls_tls().build()?;
    let mut json_headers = HeaderMap::new();
    json_headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    let response = client
      .get("https://ipinfo.io/json/")
      .headers(json_headers)
      .send()?
      .error_for_status()?;
    Ok(response.json()?)
  }
}

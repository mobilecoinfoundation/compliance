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
    let data = response.text()?;
    let data_json: serde_json::Value = serde_json::from_str(&data)?;

    let data_missing_err = Err(ConfigError::DataMissing(data_json.to_string()));
    let country: &str = match data_json["country"].as_str() {
      Some(c) => c,
      None => return data_missing_err,
    };
    let region: &str = match data_json["region"].as_str() {
      Some(r) => r,
      None => return data_missing_err,
    };

    Ok(Location {
      country_code: String::from(country),
      region: String::from(region),
    })
  }
}

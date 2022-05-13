use crate::{ConfigError, Location, LocationProvider};
use reqwest::{
  blocking::Client,
  header::{HeaderMap, HeaderValue, CONTENT_TYPE},
};

// Fetch from ipwho.is
pub struct IpWhoIs;

impl LocationProvider for IpWhoIs {
  fn location(&self) -> Result<Location, ConfigError> {
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
    let data = response.text()?;
    let data_json: serde_json::Value = serde_json::from_str(&data)?;

    let data_missing_err = Err(ConfigError::DataMissing(data_json.to_string()));
    let country = data_json["country_code"]
      .as_str()
      .ok_or_else(|| ConfigError::DataMissing(data_json.to_string()))?;
    let region: &str = match data_json["region_code"].as_str() {
      Some(r) => r,
      None => return data_missing_err,
    };

    Ok(Location {
      country_code: String::from(country),
      region: String::from(region),
    })
  }
}

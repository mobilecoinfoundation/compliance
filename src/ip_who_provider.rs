#[cfg(feature = "ip_who_provider")]
mod ip_who_provider {
  use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderValue, CONTENT_TYPE},
  };
  use crate::{ConfigError, IpWhoIs, Location, LocationProvider};

  impl LocationProvider for IpWhoIs {
    fn ip_info_fetcher(&self) -> Result<Location, ConfigError> {
      let client = Client::builder().gzip(true).use_rustls_tls().build()?;
      let mut json_headers = HeaderMap::new();
      json_headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
      let response = client
        .get("https://ipwho.is/")
        .headers(json_headers)
        .send()?
        .error_for_status()?;
      let data = response.text()?;
      let data_json: serde_json::Value = serde_json::from_str(&data)?;

      let data_missing_err = Err(ConfigError::DataMissing(data_json.to_string()));
      let country = data_json["country_code"].as_str()
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
}
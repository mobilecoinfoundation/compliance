#![deny(missing_docs)]

//! MC compliance check

/// Compliance
pub mod mc_compliance {
  use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderValue, CONTENT_TYPE},
  };

  /// Error type.
  #[derive(Debug)]
  pub enum ConfigError {
    /// Error parsing json {0}
    Json(serde_json::Error),

    /// Error handling reqwest {0}
    Reqwest(reqwest::Error),

    /// Invalid country
    InvalidCountry,

    /// Data missing in the response {0}
    DataMissing(String),

    /// Unable to fetch
    UnableToFetch,
  }

  impl From<serde_json::Error> for ConfigError {
    fn from(e: serde_json::Error) -> Self {
      Self::Json(e)
    }
  }

  impl From<reqwest::Error> for ConfigError {
    fn from(e: reqwest::Error) -> Self {
      Self::Reqwest(e)
    }
  }

  struct HostInfo {
    country_code: String,
    region: String,
  }

  trait HostInfoFetcher {
    fn ip_info_fetcher(&self) -> Result<HostInfo, ConfigError>;
  }

  // Fetch from ipinfo.io
  struct IpInfoIoFetch;

  // Fetch from ipwho.is
  struct IpWhoIs;

  impl HostInfoFetcher for IpWhoIs {
    fn ip_info_fetcher(&self) -> Result<HostInfo, ConfigError> {
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
      let country: &str = match data_json["country_code"].as_str() {
        Some(c) => c,
        None => return data_missing_err,
      };
      let region: &str = match data_json["region_code"].as_str() {
        Some(r) => r,
        None => return data_missing_err,
      };

      Ok(HostInfo {
        country_code: String::from(country),
        region: String::from(region),
      })
    }
  }

  impl HostInfoFetcher for IpInfoIoFetch {
    fn ip_info_fetcher(&self) -> Result<HostInfo, ConfigError> {
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

      Ok(HostInfo {
        country_code: String::from(country),
        region: String::from(region),
      })
    }
  }

  /// https://icanhazip.com/ - returns only IP, not enough
  /// https://api.iplocation.net/?ip=8.8.8.8 - country only, no region
  /// https://ipbase.com/ - requires free plan sign-up

  /// Validates
  pub fn validate_host() -> Result<(), ConfigError> {
    const FETCHERS: [&'static dyn HostInfoFetcher; 2] = [
      &IpInfoIoFetch {},
      &IpWhoIs {}
    ];

    for fetcher in FETCHERS {
      match fetcher.ip_info_fetcher() {
        Ok(hi) => return validate_country_code(&hi.country_code, &hi.region),
        _ => continue, // try next fetcher
      }
    }

    Err(ConfigError::UnableToFetch)
  }

  /// Validates country code
  pub fn validate_country_code(country_code: &str, region: &str) -> Result<(), ConfigError> {
    let err = Err(ConfigError::InvalidCountry);
    match country_code {
      "IR" | "SY" | "CU" | "KP" => err,
      "UA" => match region {
        "Crimea" => err,
        _ => Ok(()),
      },
      _ => Ok(()),
    }
  }
}

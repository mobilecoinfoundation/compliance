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

/// Location data
pub struct Location {
  /// Country code
  pub country_code: String,
  /// Region
  pub region: String,
}

/// Location provider
pub trait LocationProvider {

  /// Location fetcher
  fn ip_info_fetcher(&self) -> Result<Location, ConfigError>;
}
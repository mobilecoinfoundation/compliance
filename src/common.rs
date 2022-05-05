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

impl PartialEq<ConfigError> for ConfigError {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (&ConfigError::InvalidCountry, &ConfigError::InvalidCountry) => true,
      (&ConfigError::Json(ref a), &ConfigError::Json(ref b)) => a.to_string() == b.to_string(),
      (&ConfigError::UnableToFetch, &ConfigError::UnableToFetch) => true,
      (&ConfigError::Reqwest(ref a), &ConfigError::Reqwest(ref b)) => a.status() == b.status(),
      _ => false,
    }
  }
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
#[derive(Clone)]
pub struct Location {
  /// Country code
  pub country_code: String,
  /// Region
  pub region: String,
}

/// Location provider
pub trait LocationProvider {
  /// Location fetcher
  fn location(&self) -> Result<Location, ConfigError>;
}

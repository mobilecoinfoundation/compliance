// Copyright (c) 2022 MobileCoin Foundation

use crate::Configuration;

/// Error type
#[derive(Debug)]
pub enum Error {
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

impl PartialEq<Error> for Error {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (&Error::InvalidCountry, &Error::InvalidCountry) => true,
      (&Error::Json(ref a), &Error::Json(ref b)) => a.to_string() == b.to_string(),
      (&Error::UnableToFetch, &Error::UnableToFetch) => true,
      (&Error::Reqwest(ref a), &Error::Reqwest(ref b)) => a.status() == b.status(),
      _ => false,
    }
  }
}

impl From<serde_json::Error> for Error {
  fn from(e: serde_json::Error) -> Self {
    Self::Json(e)
  }
}

impl From<reqwest::Error> for Error {
  fn from(e: reqwest::Error) -> Self {
    Self::Reqwest(e)
  }
}

/// Location data
#[derive(Clone, serde::Deserialize)]
pub struct Location {
  /// Country code
  #[serde(alias = "country")]
  pub country_code: String,
  /// Region
  #[serde(alias = "region")]
  pub region_code: String,
}

/// Location provider
pub trait LocationProvider {
  /// Location fetcher
  fn location(&self, config: Option<&Configuration>) -> Result<Location, Error>;
}

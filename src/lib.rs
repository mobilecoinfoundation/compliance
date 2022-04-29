#![deny(missing_docs)]

//! MC compliance check

/// Compliance

mod common;
pub use common::{ConfigError, Location, LocationProvider};

mod ip_info_provider;
mod ip_who_provider;

// Fetch from ipinfo.io
struct IpInfoIoFetch;

// Fetch from ipwho.is
struct IpWhoIs;

/// https://icanhazip.com/ - returns only IP, not enough
/// https://api.iplocation.net/?ip=8.8.8.8 - country only, no region
/// https://ipbase.com/ - requires free plan sign-up

/// Validates
pub fn validate_host() -> Result<(), ConfigError> {
  const FETCHERS: [&'static dyn LocationProvider; 2] = [
    #[cfg(feature = "ip_info_provider")]
    &IpInfoIoFetch {},
    #[cfg(feature = "ip_who_provider")]
    &IpWhoIs {}
  ];

  for fetcher in FETCHERS {
    match fetcher.ip_info_fetcher() {
      Ok(location) => return validate_country_code(&location),
      _ => continue, // try next fetcher
    }
  }

  Err(ConfigError::UnableToFetch)
}

/// Validates country code
pub fn validate_country_code(location: &Location) -> Result<(), ConfigError> {
  let err = Err(ConfigError::InvalidCountry);
  match location.country_code.as_str() {
    "IR" | "SY" | "CU" | "KP" => err,
    "UA" => match location.region.as_str() {
      "Crimea" => err,
      _ => Ok(()),
    },
    _ => Ok(()),
  }
}

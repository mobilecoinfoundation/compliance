// Copyright (c) 2022 MobileCoin Foundation

#![deny(missing_docs)]

//! MC compliance check

/// Compliance
mod common;

pub use common::{ConfigError, Location, LocationProvider};

mod ip_info;
mod ip_who;
mod us_ofac;

#[cfg(feature = "ip_info_provider")]
use crate::ip_info::IpInfoIoFetch;
#[cfg(feature = "ip_who_provider")]
use crate::ip_who::IpWhoIs;
use crate::us_ofac::validate_country_code;

fn get_providers() -> Vec<Box<dyn LocationProvider>> {
  let mut providers: Vec<Box<dyn LocationProvider>> = vec![];

  #[cfg(feature = "ip_info_provider")]
  providers.push(Box::new(IpInfoIoFetch {}));

  #[cfg(feature = "ip_who_provider")]
  providers.push(Box::new(IpWhoIs {}));

  providers
}

// Note: rejected options
// https://icanhazip.com/ - returns only IP, not enough
// https://api.iplocation.net/?ip=8.8.8.8 - country only, no region
// https://ipbase.com/ - requires free plan sign-up

/// Validates
pub fn validate_host() -> Result<(), ConfigError> {
  let providers = get_providers();
  for provider in providers {
    match provider.location() {
      Ok(location) => return validate_country_code(&location),
      _ => continue, // try next fetcher
    }
  }

  Err(ConfigError::UnableToFetch)
}

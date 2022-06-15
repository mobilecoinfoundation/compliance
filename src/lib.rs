// Copyright (c) 2022 MobileCoin Foundation

#![deny(missing_docs)]

//! MC compliance check

/// Compliance
mod common;

use std::borrow::Borrow;
pub use common::{Error, Location, LocationProvider};
use crate::configuration::Configuration;

#[cfg(feature = "ip_info_provider")]
mod ip_info;
#[cfg(feature = "ip_who_provider")]
mod ip_who;
mod us_ofac;
mod configuration;

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

///
///
pub struct ComplianceChecker {
  config: Option<Configuration>,
}

impl ComplianceChecker {
  ///
  /// Creates an instance of the Compliance Checker
  pub fn new(config: Option<Configuration>) -> Self {
    Self {
      config
    }
  }

  /// Validates
  pub fn validate_host(&self) -> Result<(), Error> {
    let providers = get_providers();
    for provider in providers {
      match provider.location(&self.config) {
        Ok(location) => return validate_country_code(&location),
        _ => continue, // try next fetcher
      }
    }

    Err(Error::UnableToFetch)
  }
}

#[cfg(feature = "usa_local_test")]
mod tests {
  #[test]
  fn usa_test() {
    let checker = crate::ComplianceChecker::new(None);

    assert_eq!(
      checker.validate_host(),
      Ok(())
    )
  }
}


use crate::{ConfigError, Location};
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::{HashMap, HashSet};

fn get_non_passing_entries() -> HashMap<String, Option<HashSet<String>>> {
  HashMap::from([
    (
      String::from("UA"),
      Some(HashSet::from([String::from("Crimea")])),
    ),
    (String::from("IR"), None),
    (String::from("SY"), None),
    (String::from("CU"), None),
    (String::from("KP"), None),
  ])
}

/// Validates country code
pub fn validate_country_code(location: &Location) -> Result<(), ConfigError> {
  let err = Err(ConfigError::InvalidCountry);
  match get_non_passing_entries().entry(location.clone().country_code) {
    Vacant(_) => Ok(()),
    Occupied(entry) => match entry.get() {
      Some(region) => match region.get(location.region.as_str()) {
        Some(_) => err,
        None => Ok(()),
      },
      None => err,
    },
  }
}

#[cfg(test)]
mod tests {
  use crate::us_ofac::get_non_passing_entries;
  use crate::{validate_country_code, ConfigError, Location};

  #[test]
  fn test_validate_passing_country() {
    assert_eq!(
      validate_country_code(&Location {
        country_code: String::from("US"),
        region: String::from(""),
      }),
      Ok(())
    )
  }

  #[test]
  fn test_validate_non_passing_country_without_region() {
    assert_eq!(
      validate_country_code(&Location {
        country_code: String::from("CU"),
        region: String::from(""),
      }),
      Err(ConfigError::InvalidCountry)
    )
  }

  #[test]
  fn test_validate_passing_country_with_non_passing_region() {
    assert_eq!(
      validate_country_code(&Location {
        country_code: String::from("UA"),
        region: String::from("Crimea"),
      }),
      Err(ConfigError::InvalidCountry)
    )
  }

  #[test]
  fn test_validate_passing_country_with_passing_region() {
    assert_eq!(
      validate_country_code(&Location {
        country_code: String::from("UA"),
        region: String::from("SomeRegion"),
      }),
      Ok(())
    )
  }

  #[test]
  fn test_validate_all_non_passing_countries() {
    let error = Err(ConfigError::InvalidCountry);
    for country in get_non_passing_entries() {
      match country.clone().1 {
        Some(regions) => {
          for region in regions {
            let result = validate_country_code(&Location {
              country_code: country.clone().0,
              region: region.clone(),
            });
            assert_eq!(
              result, error,
              "Testing country {} and region {}",
              country.0, region
            );
          }
        }
        _ => {}
      }
    }
  }
}

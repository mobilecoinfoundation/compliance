use crate::{ConfigError, Location};
use std::collections::{HashMap, HashSet};

fn get_non_passing_entries() -> HashMap<String, Option<HashSet<String>>> {
  HashMap::from([
    (
      "UA".into(),
      Some(HashSet::from(["Crimea".into()])),
    ),
    ("IR".into(), None),
    ("SY".into(), None),
    ("CU".into(), None),
    ("KP".into(), None),
  ])
}

/// Validates country code
pub fn validate_country_code(location: &Location) -> Result<(), ConfigError> {
  let err = Err(ConfigError::InvalidCountry);
  match get_non_passing_entries().get(&location.country_code) {
    Some(regions) => match regions {
      Some(regions) => match regions.get(&location.region) {
        Some(_) => err,
        None => Ok(())
      },
      None => err,
    },
    None => Ok(()),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_validate_passing_country() {
    assert_eq!(
      validate_country_code(&Location {
        country_code: "US".into(),
        region: "".into(),
      }),
      Ok(())
    )
  }

  #[test]
  fn test_validate_non_passing_country_without_region() {
    assert_eq!(
      validate_country_code(&Location {
        country_code: "CU".into(),
        region: "".into(),
      }),
      Err(ConfigError::InvalidCountry)
    )
  }

  #[test]
  fn test_validate_passing_country_with_non_passing_region() {
    assert_eq!(
      validate_country_code(&Location {
        country_code: "UA".into(),
        region: "Crimea".into(),
      }),
      Err(ConfigError::InvalidCountry)
    )
  }

  #[test]
  fn test_validate_passing_country_with_passing_region() {
    assert_eq!(
      validate_country_code(&Location {
        country_code: "UA".into(),
        region: "SomeRegion".into(),
      }),
      Ok(())
    )
  }

  #[test]
  fn test_validate_all_non_passing_countries() {
    let error = Err(ConfigError::InvalidCountry);
    for (country, regions) in get_non_passing_entries() {
      if let Some(regions) = regions {
        for region in regions {
          let result = validate_country_code(&Location {
            country_code: country.clone(),
            region: region.clone(),
          });
          assert_eq!(
            result, error,
            "Unexpected result for country {} and region {}",
            country, region
          );
        }
      }
    }
  }
}

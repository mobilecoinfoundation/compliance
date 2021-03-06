// Copyright (c) 2022 The MobileCoin Foundation

use crate::{Configuration, Error, Location, LocationProvider};
use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderValue, CONTENT_TYPE},
};

// Fetch from ipwho.is
pub struct IpWhoIs;

impl LocationProvider for IpWhoIs {
    fn location(&self, _: Option<&Configuration>) -> Result<Location, Error> {
        let mut json_headers = HeaderMap::new();
        json_headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        let client = Client::builder()
            .default_headers(json_headers)
            .gzip(true)
            .use_rustls_tls()
            .build()?;
        let response = client.get("https://ipwho.is/").send()?.error_for_status()?;
        Ok(response.json()?)
    }
}

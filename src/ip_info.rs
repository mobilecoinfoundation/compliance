// Copyright (c) 2022 MobileCoin Foundation

use crate::{Configuration, Error, Location, LocationProvider};
use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderValue, CONTENT_TYPE},
};

pub struct IpInfoIoFetch;

impl LocationProvider for IpInfoIoFetch {
    fn location(&self, config: Option<&Configuration>) -> Result<Location, Error> {
        let mut json_headers = HeaderMap::new();
        json_headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        let client = Client::builder()
            .default_headers(json_headers)
            .gzip(true)
            .use_rustls_tls()
            .build()?;

        let mut suffix = String::new();
        if let Some(config) = config {
            if let Some(key) = &config.ip_info_key {
                suffix = format!("?token={}", key);
            }
        }
        let url = format!("https://ipinfo.io/json/{}", suffix);
        let response = client.get(url).send()?.error_for_status()?;
        Ok(response.json()?)
    }
}

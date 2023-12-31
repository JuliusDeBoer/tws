use hyper::{header, HeaderMap};
use std::env;

pub fn parse_url(url: String) -> String {
    url.replace('\\', "/").replace("//", "/")
}

pub fn set_default_headers(headers: &mut HeaderMap<header::HeaderValue>) {
    let version: String = format!("tws/{}", env!("CARGO_PKG_VERSION"));

    headers.insert(
        header::SERVER,
        header::HeaderValue::from_str(&version).unwrap(),
    );

    headers.insert(
        header::ACCESS_CONTROL_ALLOW_ORIGIN,
        header::HeaderValue::from_static("*"),
    );

    headers.insert(
        header::CACHE_CONTROL,
        header::HeaderValue::from_static("no-cache"),
    );
}

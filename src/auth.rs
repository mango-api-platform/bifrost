use warp::http::HeaderMap;

pub fn with_auth(headers: &HeaderMap) -> Option<String> {
    if let Some(api_key_header) = headers.get("Authorization") {
        let api_key = api_key_header.to_str().ok()?;
        if api_key == "Bearer my-key" {
            return Some(api_key.to_string());
        }
    }
    None
}

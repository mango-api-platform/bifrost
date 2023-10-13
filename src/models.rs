use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RouteInfo {
    pub route: String,
    pub internal_endpoint: String,
    pub is_auth: bool,
}

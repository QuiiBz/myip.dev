use std::net::IpAddr;

use axum::http::HeaderValue;
use serde::Serialize;

pub const X_REAL_IP: &str = "X-Real-Ip";
pub const X_REAL_PROTO: &str = "X-Real-Proto";
pub const X_TLS_VERSION: &str = "X-Tls-Version";

#[derive(Debug, Serialize)]
pub struct Http {
    version: String,
    tls: String,
    user_agent: Option<String>,
}

impl Http {
    pub fn new(version: String, tls: String, user_agent: Option<String>) -> Self {
        Self {
            version,
            tls,
            user_agent,
        }
    }
}

pub fn is_user_agent_automated(user_agent: &Option<String>) -> bool {
    return match user_agent {
        None => false,
        Some(user_agent) => {
            return user_agent.starts_with("curl/")
                || user_agent.starts_with("Wget/")
                || user_agent.starts_with("HTTPie/");
        }
    };
}

pub fn extract_ip(ip_header: Option<&HeaderValue>, fallback_ip: IpAddr) -> String {
    ip_header.map_or_else(
        || fallback_ip.to_string(),
        |x_real_ip| x_real_ip.to_str().unwrap_or_default().to_string(),
    )
}

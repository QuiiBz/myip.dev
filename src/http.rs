use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Http {
    version: String,
    user_agent: Option<String>,
}

impl Http {
    pub fn new(version: String, user_agent: Option<String>) -> Self {
        Self {
            version,
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

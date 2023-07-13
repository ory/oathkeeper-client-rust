/*
 * ORY Oathkeeper
 *
 * ORY Oathkeeper is a reverse proxy that checks the HTTP Authorization for validity against a set of rules. This service uses Hydra to validate access tokens and policies.
 *
 * The version of the OpenAPI document: v0.40.4
 * Contact: hi@ory.am
 * Generated by: https://openapi-generator.tech
 */

/// GenericError : The standard error format



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GenericError {
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<i64>,
    #[serde(rename = "details", skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<::std::collections::HashMap<String, serde_json::Value>>>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "request", skip_serializing_if = "Option::is_none")]
    pub request: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl Default for GenericError {
    fn default() -> Self {
        Self::new()
    }
}

impl GenericError {
    /// The standard error format
    pub fn new() -> GenericError {
        GenericError {
                code: None,
                details: None,
                message: None,
                reason: None,
                request: None,
                status: None,
        }
    }
}



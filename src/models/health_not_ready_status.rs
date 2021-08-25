/*
 * ORY Oathkeeper
 *
 * ORY Oathkeeper is a reverse proxy that checks the HTTP Authorization for validity against a set of rules. This service uses Hydra to validate access tokens and policies.
 *
 * The version of the OpenAPI document: v0.0.0-alpha.62
 * Contact: hi@ory.am
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HealthNotReadyStatus {
    /// Errors contains a list of errors that caused the not ready status.
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<::std::collections::HashMap<String, String>>,
}

impl HealthNotReadyStatus {
    pub fn new() -> HealthNotReadyStatus {
        HealthNotReadyStatus {
            errors: None,
        }
    }
}



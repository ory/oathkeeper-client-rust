/*
 * ORY Oathkeeper
 *
 * ORY Oathkeeper is a reverse proxy that checks the HTTP Authorization for validity against a set of rules. This service uses Hydra to validate access tokens and policies.
 *
 * The version of the OpenAPI document: v0.0.0-alpha.59
 * Contact: hi@ory.am
 * Generated by: https://openapi-generator.tech
 */

/// ListRulesUnauthorized : The standard error format



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListRulesUnauthorized {
    #[serde(rename = "Payload", skip_serializing_if = "Option::is_none")]
    pub payload: Option<Box<crate::models::ListRulesUnauthorizedBody>>,
}

impl ListRulesUnauthorized {
    /// The standard error format
    pub fn new() -> ListRulesUnauthorized {
        ListRulesUnauthorized {
            payload: None,
        }
    }
}



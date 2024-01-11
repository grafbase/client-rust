/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.5.1
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

/// IdentityWithCredentials : Create Identity and Import Credentials



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentityWithCredentials {
    #[serde(rename = "oidc", skip_serializing_if = "Option::is_none")]
    pub oidc: Option<Box<crate::models::IdentityWithCredentialsOidc>>,
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<Box<crate::models::IdentityWithCredentialsPassword>>,
}

impl Default for IdentityWithCredentials {
    fn default() -> Self {
        Self::new()
    }
}

impl IdentityWithCredentials {
    /// Create Identity and Import Credentials
    pub fn new() -> IdentityWithCredentials {
        IdentityWithCredentials {
                oidc: None,
                password: None,
        }
    }
}



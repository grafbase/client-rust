/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.6.1
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

/// IdentityWithCredentialsPasswordConfig : Create Identity and Import Password Credentials Configuration



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentityWithCredentialsPasswordConfig {
    /// The hashed password in [PHC format](https://www.ory.sh/docs/kratos/manage-identities/import-user-accounts-identities#hashed-passwords)
    #[serde(rename = "hashed_password", skip_serializing_if = "Option::is_none")]
    pub hashed_password: Option<String>,
    /// The password in plain text if no hash is available.
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

impl Default for IdentityWithCredentialsPasswordConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl IdentityWithCredentialsPasswordConfig {
    /// Create Identity and Import Password Credentials Configuration
    pub fn new() -> IdentityWithCredentialsPasswordConfig {
        IdentityWithCredentialsPasswordConfig {
                hashed_password: None,
                password: None,
        }
    }
}



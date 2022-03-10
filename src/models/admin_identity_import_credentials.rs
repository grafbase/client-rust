/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v0.0.1-alpha.124
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdminIdentityImportCredentials {
    #[serde(rename = "oidc", skip_serializing_if = "Option::is_none")]
    pub oidc: Option<Box<crate::models::AdminCreateIdentityImportCredentialsOidc>>,
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<Box<crate::models::AdminCreateIdentityImportCredentialsPassword>>,
}

impl AdminIdentityImportCredentials {
    pub fn new() -> AdminIdentityImportCredentials {
        AdminIdentityImportCredentials {
            oidc: None,
            password: None,
        }
    }
}



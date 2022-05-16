/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v0.0.1-alpha.175
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdminCreateIdentityImportCredentialsOidc {
    #[serde(rename = "config", skip_serializing_if = "Option::is_none")]
    pub config: Option<Box<crate::models::AdminCreateIdentityImportCredentialsOidcConfig>>,
}

impl AdminCreateIdentityImportCredentialsOidc {
    pub fn new() -> AdminCreateIdentityImportCredentialsOidc {
        AdminCreateIdentityImportCredentialsOidc {
            config: None,
        }
    }
}



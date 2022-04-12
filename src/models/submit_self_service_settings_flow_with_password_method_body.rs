/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v0.0.1-alpha.163
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubmitSelfServiceSettingsFlowWithPasswordMethodBody {
    /// CSRFToken is the anti-CSRF token
    #[serde(rename = "csrf_token", skip_serializing_if = "Option::is_none")]
    pub csrf_token: Option<String>,
    /// Method  Should be set to password when trying to update a password.
    #[serde(rename = "method")]
    pub method: String,
    /// Password is the updated password
    #[serde(rename = "password")]
    pub password: String,
}

impl SubmitSelfServiceSettingsFlowWithPasswordMethodBody {
    pub fn new(method: String, password: String) -> SubmitSelfServiceSettingsFlowWithPasswordMethodBody {
        SubmitSelfServiceSettingsFlowWithPasswordMethodBody {
            csrf_token: None,
            method,
            password,
        }
    }
}



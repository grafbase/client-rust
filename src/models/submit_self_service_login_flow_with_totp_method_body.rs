/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v0.0.1-alpha.122
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubmitSelfServiceLoginFlowWithTotpMethodBody {
    /// Sending the anti-csrf token is only required for browser login flows.
    #[serde(rename = "csrf_token", skip_serializing_if = "Option::is_none")]
    pub csrf_token: Option<String>,
    /// Method should be set to \"totp\" when logging in using the TOTP strategy.
    #[serde(rename = "method")]
    pub method: String,
    /// The TOTP code.
    #[serde(rename = "totp_code")]
    pub totp_code: String,
}

impl SubmitSelfServiceLoginFlowWithTotpMethodBody {
    pub fn new(method: String, totp_code: String) -> SubmitSelfServiceLoginFlowWithTotpMethodBody {
        SubmitSelfServiceLoginFlowWithTotpMethodBody {
            csrf_token: None,
            method,
            totp_code,
        }
    }
}



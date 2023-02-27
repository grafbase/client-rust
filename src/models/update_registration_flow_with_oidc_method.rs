/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.1.21
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

/// UpdateRegistrationFlowWithOidcMethod : Update Registration Flow with OpenID Connect Method



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateRegistrationFlowWithOidcMethod {
    /// The CSRF Token
    #[serde(rename = "csrf_token", skip_serializing_if = "Option::is_none")]
    pub csrf_token: Option<String>,
    /// Method to use  This field must be set to `oidc` when using the oidc method.
    #[serde(rename = "method")]
    pub method: String,
    /// The provider to register with
    #[serde(rename = "provider")]
    pub provider: String,
    /// The identity traits
    #[serde(rename = "traits", skip_serializing_if = "Option::is_none")]
    pub traits: Option<serde_json::Value>,
    /// Transient data to pass along to any webhooks
    #[serde(rename = "transient_payload", skip_serializing_if = "Option::is_none")]
    pub transient_payload: Option<serde_json::Value>,
}


impl UpdateRegistrationFlowWithOidcMethod {
    /// Update Registration Flow with OpenID Connect Method
    pub fn new(method: String, provider: String) -> UpdateRegistrationFlowWithOidcMethod {
        UpdateRegistrationFlowWithOidcMethod {
                csrf_token: None,
                method,
                provider,
                traits: None,
                transient_payload: None,
        }
    }
}



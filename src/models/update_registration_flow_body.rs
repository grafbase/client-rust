/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.2.10
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

/// UpdateRegistrationFlowBody : Update Registration Request Body


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "method")]
pub enum UpdateRegistrationFlowBody {
    #[serde(rename="code")]
    UpdateRegistrationFlowWithCodeMethod {
        /// The OTP Code sent to the user
        #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
        // false, false, , String, false
        code: Option<String>,
        /// The CSRF Token
        #[serde(rename = "csrf_token", skip_serializing_if = "Option::is_none")]
        // false, false, , String, false
        csrf_token: Option<String>,
        /// Resend restarts the flow with a new code
        #[serde(rename = "resend", skip_serializing_if = "Option::is_none")]
        // false, false, , String, false
        resend: Option<String>,
        /// The identity's traits
        #[serde(rename = "traits")]
        // true, false, , serde_json::Value, false
        traits: serde_json::Value,
        /// Transient data to pass along to any webhooks
        #[serde(rename = "transient_payload", skip_serializing_if = "Option::is_none")]
        // false, false, , serde_json::Value, false
        transient_payload: Option<serde_json::Value>,
    },
    #[serde(rename="oidc")]
    UpdateRegistrationFlowWithOidcMethod {
        /// The CSRF Token
        #[serde(rename = "csrf_token", skip_serializing_if = "Option::is_none")]
        // false, false, , String, false
        csrf_token: Option<String>,
        /// IDToken is an optional id token provided by an OIDC provider  If submitted, it is verified using the OIDC provider's public key set and the claims are used to populate the OIDC credentials of the identity. If the OIDC provider does not store additional claims (such as name, etc.) in the IDToken itself, you can use the `traits` field to populate the identity's traits. Note, that Apple only includes the users email in the IDToken.  Supported providers are Apple
        #[serde(rename = "id_token", skip_serializing_if = "Option::is_none")]
        // false, false, , String, false
        id_token: Option<String>,
        /// IDTokenNonce is the nonce, used when generating the IDToken. If the provider supports nonce validation, the nonce will be validated against this value and is required.
        #[serde(rename = "id_token_nonce", skip_serializing_if = "Option::is_none")]
        // false, false, , String, false
        id_token_nonce: Option<String>,
        /// The provider to register with
        #[serde(rename = "provider")]
        // true, false, , String, false
        provider: String,
        /// The identity traits
        #[serde(rename = "traits", skip_serializing_if = "Option::is_none")]
        // false, false, , serde_json::Value, false
        traits: Option<serde_json::Value>,
        /// Transient data to pass along to any webhooks
        #[serde(rename = "transient_payload", skip_serializing_if = "Option::is_none")]
        // false, false, , serde_json::Value, false
        transient_payload: Option<serde_json::Value>,
        /// UpstreamParameters are the parameters that are passed to the upstream identity provider.  These parameters are optional and depend on what the upstream identity provider supports. Supported parameters are: `login_hint` (string): The `login_hint` parameter suppresses the account chooser and either pre-fills the email box on the sign-in form, or selects the proper session. `hd` (string): The `hd` parameter limits the login/registration process to a Google Organization, e.g. `mycollege.edu`. `prompt` (string): The `prompt` specifies whether the Authorization Server prompts the End-User for reauthentication and consent, e.g. `select_account`.
        #[serde(rename = "upstream_parameters", skip_serializing_if = "Option::is_none")]
        // false, false, , serde_json::Value, false
        upstream_parameters: Option<serde_json::Value>,
    },
    #[serde(rename="password")]
    UpdateRegistrationFlowWithPasswordMethod {
        /// The CSRF Token
        #[serde(rename = "csrf_token", skip_serializing_if = "Option::is_none")]
        // false, false, , String, false
        csrf_token: Option<String>,
        /// Password to sign the user up with
        #[serde(rename = "password")]
        // true, false, , String, false
        password: String,
        /// The identity's traits
        #[serde(rename = "traits")]
        // true, false, , serde_json::Value, false
        traits: serde_json::Value,
        /// Transient data to pass along to any webhooks
        #[serde(rename = "transient_payload", skip_serializing_if = "Option::is_none")]
        // false, false, , serde_json::Value, false
        transient_payload: Option<serde_json::Value>,
    },
    #[serde(rename="webauthn")]
    UpdateRegistrationFlowWithWebAuthnMethod {
        /// CSRFToken is the anti-CSRF token
        #[serde(rename = "csrf_token", skip_serializing_if = "Option::is_none")]
        // false, false, , String, false
        csrf_token: Option<String>,
        /// The identity's traits
        #[serde(rename = "traits")]
        // true, false, , serde_json::Value, false
        traits: serde_json::Value,
        /// Transient data to pass along to any webhooks
        #[serde(rename = "transient_payload", skip_serializing_if = "Option::is_none")]
        // false, false, , serde_json::Value, false
        transient_payload: Option<serde_json::Value>,
        /// Register a WebAuthn Security Key  It is expected that the JSON returned by the WebAuthn registration process is included here.
        #[serde(rename = "webauthn_register", skip_serializing_if = "Option::is_none")]
        // false, false, , String, false
        webauthn_register: Option<String>,
        /// Name of the WebAuthn Security Key to be Added  A human-readable name for the security key which will be added.
        #[serde(rename = "webauthn_register_displayname", skip_serializing_if = "Option::is_none")]
        // false, false, , String, false
        webauthn_register_displayname: Option<String>,
    },
}






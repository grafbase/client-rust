/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.9.0
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "method")]
pub enum UpdateLoginFlowBody {
    #[serde(rename="code")]
    UpdateLoginFlowWithCodeMethod {
        /// Code is the 6 digits code sent to the user
        #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
        // false, false, , String, false
        code: Option<String>,
        /// CSRFToken is the anti-CSRF token
        #[serde(rename = "csrf_token")]
        // true, false, , String, false
        csrf_token: String,
        /// Identifier is the code identifier The identifier requires that the user has already completed the registration or settings with code flow.
        #[serde(rename = "identifier", skip_serializing_if = "Option::is_none")]
        // false, false, , String, false
        identifier: Option<String>,
        /// Resend is set when the user wants to resend the code
        #[serde(rename = "resend", skip_serializing_if = "Option::is_none")]
        // false, false, , String, false
        resend: Option<String>,
        /// Transient data to pass along to any webhooks
        #[serde(rename = "transient_payload", skip_serializing_if = "Option::is_none")]
        // false, false, , serde_json::Value, false
        transient_payload: Option<serde_json::Value>,
    },
    #[serde(rename="lookup_secret")]
    UpdateLoginFlowWithLookupSecretMethod {
        /// Sending the anti-csrf token is only required for browser login flows.
        #[serde(rename = "csrf_token", skip_serializing_if = "Option::is_none")]
        // false, false, , String, false
        csrf_token: Option<String>,
        /// The lookup secret.
        #[serde(rename = "lookup_secret")]
        // true, false, , String, false
        lookup_secret: String,
    },
    #[serde(rename="oidc")]
    UpdateLoginFlowWithOidcMethod {
        /// The CSRF Token
        #[serde(rename = "csrf_token", skip_serializing_if = "Option::is_none")]
        // false, false, , String, false
        csrf_token: Option<String>,
        /// IDToken is an optional id token provided by an OIDC provider  If submitted, it is verified using the OIDC provider's public key set and the claims are used to populate the OIDC credentials of the identity. If the OIDC provider does not store additional claims (such as name, etc.) in the IDToken itself, you can use the `traits` field to populate the identity's traits. Note, that Apple only includes the users email in the IDToken.  Supported providers are Apple
        #[serde(rename = "id_token", skip_serializing_if = "Option::is_none")]
        // false, false, , String, false
        id_token: Option<String>,
        /// IDTokenNonce is the nonce, used when generating the IDToken. If the provider supports nonce validation, the nonce will be validated against this value and required.
        #[serde(rename = "id_token_nonce", skip_serializing_if = "Option::is_none")]
        // false, false, , String, false
        id_token_nonce: Option<String>,
        /// The provider to register with
        #[serde(rename = "provider")]
        // true, false, , String, false
        provider: String,
        /// The identity traits. This is a placeholder for the registration flow.
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
    UpdateLoginFlowWithPasswordMethod {
        /// Sending the anti-csrf token is only required for browser login flows.
        #[serde(rename = "csrf_token", skip_serializing_if = "Option::is_none")]
        // false, false, , String, false
        csrf_token: Option<String>,
        /// Identifier is the email or username of the user trying to log in.
        #[serde(rename = "identifier")]
        // true, false, , String, false
        identifier: String,
        /// The user's password.
        #[serde(rename = "password")]
        // true, false, , String, false
        password: String,
        /// Identifier is the email or username of the user trying to log in. This field is deprecated!
        #[serde(rename = "password_identifier", skip_serializing_if = "Option::is_none")]
        // false, false, , String, false
        password_identifier: Option<String>,
        /// Transient data to pass along to any webhooks
        #[serde(rename = "transient_payload", skip_serializing_if = "Option::is_none")]
        // false, false, , serde_json::Value, false
        transient_payload: Option<serde_json::Value>,
    },
    #[serde(rename="totp")]
    UpdateLoginFlowWithTotpMethod {
        /// Sending the anti-csrf token is only required for browser login flows.
        #[serde(rename = "csrf_token", skip_serializing_if = "Option::is_none")]
        // false, false, , String, false
        csrf_token: Option<String>,
        /// The TOTP code.
        #[serde(rename = "totp_code")]
        // true, false, , String, false
        totp_code: String,
        /// Transient data to pass along to any webhooks
        #[serde(rename = "transient_payload", skip_serializing_if = "Option::is_none")]
        // false, false, , serde_json::Value, false
        transient_payload: Option<serde_json::Value>,
    },
    #[serde(rename="webauthn")]
    UpdateLoginFlowWithWebAuthnMethod {
        /// Sending the anti-csrf token is only required for browser login flows.
        #[serde(rename = "csrf_token", skip_serializing_if = "Option::is_none")]
        // false, false, , String, false
        csrf_token: Option<String>,
        /// Identifier is the email or username of the user trying to log in.
        #[serde(rename = "identifier")]
        // true, false, , String, false
        identifier: String,
        /// Transient data to pass along to any webhooks
        #[serde(rename = "transient_payload", skip_serializing_if = "Option::is_none")]
        // false, false, , serde_json::Value, false
        transient_payload: Option<serde_json::Value>,
        /// Login a WebAuthn Security Key  This must contain the ID of the WebAuthN connection.
        #[serde(rename = "webauthn_login", skip_serializing_if = "Option::is_none")]
        // false, false, , String, false
        webauthn_login: Option<String>,
    },
}






/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.2.1
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

/// LoginFlowState : The state represents the state of the login flow.  choose_method: ask the user to choose a method (e.g. login account via email) sent_email: the email has been sent to the user passed_challenge: the request was successful and the login challenge was passed.

/// The state represents the state of the login flow.  choose_method: ask the user to choose a method (e.g. login account via email) sent_email: the email has been sent to the user passed_challenge: the request was successful and the login challenge was passed.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LoginFlowState {
    #[serde(rename = "choose_method")]
    ChooseMethod,
    #[serde(rename = "sent_email")]
    SentEmail,
    #[serde(rename = "passed_challenge")]
    PassedChallenge,

}

impl ToString for LoginFlowState {
    fn to_string(&self) -> String {
        match self {
            Self::ChooseMethod => String::from("choose_method"),
            Self::SentEmail => String::from("sent_email"),
            Self::PassedChallenge => String::from("passed_challenge"),
        }
    }
}





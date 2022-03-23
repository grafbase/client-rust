/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v0.0.1-alpha.145
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectMetadata {
    /// The Project's Creation Date
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "hosts")]
    pub hosts: Vec<String>,
    #[serde(rename = "id")]
    pub id: String,
    /// The project's name if set
    #[serde(rename = "name")]
    pub name: String,
    /// The project's slug
    #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    /// The state of the project.
    #[serde(rename = "state")]
    pub state: State,
    #[serde(rename = "subscription_id", skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    /// Last Time Project was Updated
    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

impl ProjectMetadata {
    pub fn new(created_at: String, hosts: Vec<String>, id: String, name: String, state: State, updated_at: String) -> ProjectMetadata {
        ProjectMetadata {
            created_at,
            hosts,
            id,
            name,
            slug: None,
            state,
            subscription_id: None,
            updated_at,
        }
    }
}

/// The state of the project.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "halted")]
    Halted,
}


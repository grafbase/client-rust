/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v0.0.1-alpha.31
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Project {
    /// The Project's Creation Date
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "current_revision")]
    pub current_revision: Box<crate::models::ProjectRevision>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "ongoing_stripe_checkout_id", skip_serializing_if = "Option::is_none")]
    pub ongoing_stripe_checkout_id: Option<Box<crate::models::NullString>>,
    #[serde(rename = "revisions")]
    pub revisions: Vec<crate::models::ProjectRevision>,
    /// The project's slug
    #[serde(rename = "slug")]
    pub slug: String,
    /// The state of the project.
    #[serde(rename = "state")]
    pub state: State,
    #[serde(rename = "subscription_id", skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<Box<crate::models::NullUuid>>,
    /// Last Time Project was Updated
    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

impl Project {
    pub fn new(created_at: String, current_revision: crate::models::ProjectRevision, id: String, revisions: Vec<crate::models::ProjectRevision>, slug: String, state: State, updated_at: String) -> Project {
        Project {
            created_at,
            current_revision: Box::new(current_revision),
            id,
            ongoing_stripe_checkout_id: None,
            revisions,
            slug,
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


/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.4.7
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetOrganizationResponse {
    #[serde(rename = "organization")]
    pub organization: Box<crate::models::Organization>,
}


impl GetOrganizationResponse {
    pub fn new(organization: crate::models::Organization) -> GetOrganizationResponse {
        GetOrganizationResponse {
                organization: Box::new(organization),
        }
    }
}


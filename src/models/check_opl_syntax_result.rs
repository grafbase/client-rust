/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.1.10
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckOplSyntaxResult {
    /// The list of syntax errors
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<crate::models::ParseError>>,
}

impl Default for CheckOplSyntaxResult {
    fn default() -> Self {
        Self::new()
    }
}

impl CheckOplSyntaxResult {
    pub fn new() -> CheckOplSyntaxResult {
        CheckOplSyntaxResult {
                errors: None,
        }
    }
}



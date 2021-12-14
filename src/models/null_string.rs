/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v0.0.1-alpha.31
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

/// NullString : var s NullString err := db.QueryRow(\"SELECT name FROM foo WHERE id=?\", id).Scan(&s) ... if s.Valid { use s.String } else { NULL value }



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NullString {
    #[serde(rename = "String", skip_serializing_if = "Option::is_none")]
    pub string: Option<String>,
    #[serde(rename = "Valid", skip_serializing_if = "Option::is_none")]
    pub valid: Option<bool>,
}

impl NullString {
    /// var s NullString err := db.QueryRow(\"SELECT name FROM foo WHERE id=?\", id).Scan(&s) ... if s.Valid { use s.String } else { NULL value }
    pub fn new() -> NullString {
        NullString {
            string: None,
            valid: None,
        }
    }
}



/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.5.1
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

/// CreateProjectBranding : Create a Project Branding



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateProjectBranding {
    #[serde(rename = "favicon_type", skip_serializing_if = "Option::is_none")]
    pub favicon_type: Option<String>,
    #[serde(rename = "favicon_url", skip_serializing_if = "Option::is_none")]
    pub favicon_url: Option<String>,
    #[serde(rename = "logo_type", skip_serializing_if = "Option::is_none")]
    pub logo_type: Option<String>,
    #[serde(rename = "logo_url", skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "theme", skip_serializing_if = "Option::is_none")]
    pub theme: Option<Box<crate::models::ProjectBrandingColors>>,
}

impl Default for CreateProjectBranding {
    fn default() -> Self {
        Self::new()
    }
}

impl CreateProjectBranding {
    /// Create a Project Branding
    pub fn new() -> CreateProjectBranding {
        CreateProjectBranding {
                favicon_type: None,
                favicon_url: None,
                logo_type: None,
                logo_url: None,
                name: None,
                theme: None,
        }
    }
}



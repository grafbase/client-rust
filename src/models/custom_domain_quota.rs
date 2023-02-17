/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.1.17
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

/// CustomDomainQuota : Custom Domain Quota



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomDomainQuota {
    #[serde(rename = "available_domains", skip_serializing_if = "Option::is_none")]
    pub available_domains: Option<i64>,
    #[serde(rename = "can_use", skip_serializing_if = "Option::is_none")]
    pub can_use: Option<bool>,
    #[serde(rename = "used_domains", skip_serializing_if = "Option::is_none")]
    pub used_domains: Option<i64>,
}

impl Default for CustomDomainQuota {
    fn default() -> Self {
        Self::new()
    }
}

impl CustomDomainQuota {
    /// Custom Domain Quota
    pub fn new() -> CustomDomainQuota {
        CustomDomainQuota {
                available_domains: None,
                can_use: None,
                used_domains: None,
        }
    }
}



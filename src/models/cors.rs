/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.2.8
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Cors {
    /// Whether CORS is enabled for this endpoint.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// The allowed origins. Use `*` to allow all origins. A wildcard can also be used in the subdomain, i.e. `https://_*.example.com` will allow all origins on all subdomains of `example.com`.
    #[serde(rename = "origins")]
    pub origins: Vec<String>,
}


impl Cors {
    pub fn new(enabled: bool, origins: Vec<String>) -> Cors {
        Cors {
                enabled,
                origins,
        }
    }
}



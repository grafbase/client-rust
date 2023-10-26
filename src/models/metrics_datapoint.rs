/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.2.14
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

/// MetricsDatapoint : Represents a single datapoint/bucket of a time series



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricsDatapoint {
    /// The count of events that occured in this time
    #[serde(rename = "count")]
    pub count: i64,
    /// The time of the bucket
    #[serde(rename = "time")]
    pub time: String,
}


impl MetricsDatapoint {
    /// Represents a single datapoint/bucket of a time series
    pub fn new(count: i64, time: String) -> MetricsDatapoint {
        MetricsDatapoint {
                count,
                time,
        }
    }
}



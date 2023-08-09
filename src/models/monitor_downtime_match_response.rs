/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// MonitorDowntimeMatchResponse : Response for retrieving all downtime matches for a monitor.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MonitorDowntimeMatchResponse {
    /// An array of downtime matches.
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<crate::models::MonitorDowntimeMatchResponseData>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::DowntimeMeta>>,
}

impl MonitorDowntimeMatchResponse {
    /// Response for retrieving all downtime matches for a monitor.
    pub fn new() -> MonitorDowntimeMatchResponse {
        MonitorDowntimeMatchResponse {
            data: None,
            meta: None,
        }
    }
}



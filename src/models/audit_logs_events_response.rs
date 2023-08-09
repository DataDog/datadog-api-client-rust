/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// AuditLogsEventsResponse : Response object with all events matching the request and pagination information.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AuditLogsEventsResponse {
    /// Array of events matching the request.
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<crate::models::AuditLogsEvent>>,
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<Box<crate::models::AuditLogsResponseLinks>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::AuditLogsResponseMetadata>>,
}

impl AuditLogsEventsResponse {
    /// Response object with all events matching the request and pagination information.
    pub fn new() -> AuditLogsEventsResponse {
        AuditLogsEventsResponse {
            data: None,
            links: None,
            meta: None,
        }
    }
}



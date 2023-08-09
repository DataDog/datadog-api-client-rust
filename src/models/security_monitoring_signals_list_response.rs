/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// SecurityMonitoringSignalsListResponse : The response object with all security signals matching the request and pagination information.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SecurityMonitoringSignalsListResponse {
    /// An array of security signals matching the request.
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<crate::models::SecurityMonitoringSignal>>,
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<Box<crate::models::SecurityMonitoringSignalsListResponseLinks>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::SecurityMonitoringSignalsListResponseMeta>>,
}

impl SecurityMonitoringSignalsListResponse {
    /// The response object with all security signals matching the request and pagination information.
    pub fn new() -> SecurityMonitoringSignalsListResponse {
        SecurityMonitoringSignalsListResponse {
            data: None,
            links: None,
            meta: None,
        }
    }
}



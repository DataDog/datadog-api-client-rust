/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// SecurityMonitoringSignalIncidentsUpdateAttributes : Attributes describing the new list of related signals for a security signal.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SecurityMonitoringSignalIncidentsUpdateAttributes {
    /// Array of incidents that are associated with this signal.
    #[serde(rename = "incident_ids")]
    pub incident_ids: Vec<i64>,
    /// Version of the updated signal. If server side version is higher, update will be rejected.
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

impl SecurityMonitoringSignalIncidentsUpdateAttributes {
    /// Attributes describing the new list of related signals for a security signal.
    pub fn new(incident_ids: Vec<i64>) -> SecurityMonitoringSignalIncidentsUpdateAttributes {
        SecurityMonitoringSignalIncidentsUpdateAttributes {
            incident_ids,
            version: None,
        }
    }
}



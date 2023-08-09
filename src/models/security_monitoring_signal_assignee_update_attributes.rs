/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// SecurityMonitoringSignalAssigneeUpdateAttributes : Attributes describing the new assignee of a security signal.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SecurityMonitoringSignalAssigneeUpdateAttributes {
    #[serde(rename = "assignee")]
    pub assignee: Box<crate::models::SecurityMonitoringTriageUser>,
    /// Version of the updated signal. If server side version is higher, update will be rejected.
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

impl SecurityMonitoringSignalAssigneeUpdateAttributes {
    /// Attributes describing the new assignee of a security signal.
    pub fn new(assignee: crate::models::SecurityMonitoringTriageUser) -> SecurityMonitoringSignalAssigneeUpdateAttributes {
        SecurityMonitoringSignalAssigneeUpdateAttributes {
            assignee: Box::new(assignee),
            version: None,
        }
    }
}



/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// SecurityMonitoringSignalAssigneeUpdateRequest : Request body for changing the assignee of a given security monitoring signal.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SecurityMonitoringSignalAssigneeUpdateRequest {
    #[serde(rename = "data")]
    pub data: Box<crate::models::SecurityMonitoringSignalAssigneeUpdateData>,
}

impl SecurityMonitoringSignalAssigneeUpdateRequest {
    /// Request body for changing the assignee of a given security monitoring signal.
    pub fn new(data: crate::models::SecurityMonitoringSignalAssigneeUpdateData) -> SecurityMonitoringSignalAssigneeUpdateRequest {
        SecurityMonitoringSignalAssigneeUpdateRequest {
            data: Box::new(data),
        }
    }
}



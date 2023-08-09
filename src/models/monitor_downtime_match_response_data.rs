/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// MonitorDowntimeMatchResponseData : A downtime match.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MonitorDowntimeMatchResponseData {
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Box<crate::models::MonitorDowntimeMatchResponseAttributes>>,
    /// The downtime ID.
    #[serde(rename = "id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub id: Option<Option<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<crate::models::MonitorDowntimeMatchResourceType>,
}

impl MonitorDowntimeMatchResponseData {
    /// A downtime match.
    pub fn new() -> MonitorDowntimeMatchResponseData {
        MonitorDowntimeMatchResponseData {
            attributes: None,
            id: None,
            r#type: None,
        }
    }
}



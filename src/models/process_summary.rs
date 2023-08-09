/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// ProcessSummary : Process summary object.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ProcessSummary {
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Box<crate::models::ProcessSummaryAttributes>>,
    /// Process ID.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<crate::models::ProcessSummaryType>,
}

impl ProcessSummary {
    /// Process summary object.
    pub fn new() -> ProcessSummary {
        ProcessSummary {
            attributes: None,
            id: None,
            r#type: None,
        }
    }
}



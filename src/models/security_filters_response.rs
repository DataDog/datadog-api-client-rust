/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// SecurityFiltersResponse : All the available security filters objects.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SecurityFiltersResponse {
    /// A list of security filters objects.
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<crate::models::SecurityFilter>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::SecurityFilterMeta>>,
}

impl SecurityFiltersResponse {
    /// All the available security filters objects.
    pub fn new() -> SecurityFiltersResponse {
        SecurityFiltersResponse {
            data: None,
            meta: None,
        }
    }
}



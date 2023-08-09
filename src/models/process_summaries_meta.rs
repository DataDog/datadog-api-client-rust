/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// ProcessSummariesMeta : Response metadata object.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ProcessSummariesMeta {
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    pub page: Option<Box<crate::models::ProcessSummariesMetaPage>>,
}

impl ProcessSummariesMeta {
    /// Response metadata object.
    pub fn new() -> ProcessSummariesMeta {
        ProcessSummariesMeta {
            page: None,
        }
    }
}



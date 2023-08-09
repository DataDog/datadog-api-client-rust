/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// MetricBulkTagConfigCreate : Request object to bulk configure tags for metrics matching the given prefix.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MetricBulkTagConfigCreate {
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Box<crate::models::MetricBulkTagConfigCreateAttributes>>,
    /// A text prefix to match against metric names.
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "type")]
    pub r#type: crate::models::MetricBulkConfigureTagsType,
}

impl MetricBulkTagConfigCreate {
    /// Request object to bulk configure tags for metrics matching the given prefix.
    pub fn new(id: String, r#type: crate::models::MetricBulkConfigureTagsType) -> MetricBulkTagConfigCreate {
        MetricBulkTagConfigCreate {
            attributes: None,
            id,
            r#type,
        }
    }
}



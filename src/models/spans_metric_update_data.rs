/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// SpansMetricUpdateData : The new span-based metric properties.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SpansMetricUpdateData {
    #[serde(rename = "attributes")]
    pub attributes: Box<crate::models::SpansMetricUpdateAttributes>,
    #[serde(rename = "type")]
    pub r#type: crate::models::SpansMetricType,
}

impl SpansMetricUpdateData {
    /// The new span-based metric properties.
    pub fn new(attributes: crate::models::SpansMetricUpdateAttributes, r#type: crate::models::SpansMetricType) -> SpansMetricUpdateData {
        SpansMetricUpdateData {
            attributes: Box::new(attributes),
            r#type,
        }
    }
}



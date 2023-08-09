/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// SpansMetricUpdateRequest : The new span-based metric body.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SpansMetricUpdateRequest {
    #[serde(rename = "data")]
    pub data: Box<crate::models::SpansMetricUpdateData>,
}

impl SpansMetricUpdateRequest {
    /// The new span-based metric body.
    pub fn new(data: crate::models::SpansMetricUpdateData) -> SpansMetricUpdateRequest {
        SpansMetricUpdateRequest {
            data: Box::new(data),
        }
    }
}



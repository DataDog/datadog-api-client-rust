/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// ConfluentResourceRequest : The JSON:API request for updating a Confluent resource.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ConfluentResourceRequest {
    #[serde(rename = "data")]
    pub data: Box<crate::models::ConfluentResourceRequestData>,
}

impl ConfluentResourceRequest {
    /// The JSON:API request for updating a Confluent resource.
    pub fn new(data: crate::models::ConfluentResourceRequestData) -> ConfluentResourceRequest {
        ConfluentResourceRequest {
            data: Box::new(data),
        }
    }
}



/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// ConfluentAccountCreateRequest : Payload schema when adding a Confluent account.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ConfluentAccountCreateRequest {
    #[serde(rename = "data")]
    pub data: Box<crate::models::ConfluentAccountCreateRequestData>,
}

impl ConfluentAccountCreateRequest {
    /// Payload schema when adding a Confluent account.
    pub fn new(data: crate::models::ConfluentAccountCreateRequestData) -> ConfluentAccountCreateRequest {
        ConfluentAccountCreateRequest {
            data: Box::new(data),
        }
    }
}



/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// OpsgenieServiceCreateRequest : Create request for an Opsgenie service.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OpsgenieServiceCreateRequest {
    #[serde(rename = "data")]
    pub data: Box<crate::models::OpsgenieServiceCreateData>,
}

impl OpsgenieServiceCreateRequest {
    /// Create request for an Opsgenie service.
    pub fn new(data: crate::models::OpsgenieServiceCreateData) -> OpsgenieServiceCreateRequest {
        OpsgenieServiceCreateRequest {
            data: Box::new(data),
        }
    }
}



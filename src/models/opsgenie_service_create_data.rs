/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// OpsgenieServiceCreateData : Opsgenie service data for a create request.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OpsgenieServiceCreateData {
    #[serde(rename = "attributes")]
    pub attributes: Box<crate::models::OpsgenieServiceCreateAttributes>,
    #[serde(rename = "type")]
    pub r#type: crate::models::OpsgenieServiceType,
}

impl OpsgenieServiceCreateData {
    /// Opsgenie service data for a create request.
    pub fn new(attributes: crate::models::OpsgenieServiceCreateAttributes, r#type: crate::models::OpsgenieServiceType) -> OpsgenieServiceCreateData {
        OpsgenieServiceCreateData {
            attributes: Box::new(attributes),
            r#type,
        }
    }
}



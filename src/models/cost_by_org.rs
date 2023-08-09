/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// CostByOrg : Cost data.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CostByOrg {
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Box<crate::models::CostByOrgAttributes>>,
    /// Unique ID of the response.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<crate::models::CostByOrgType>,
}

impl CostByOrg {
    /// Cost data.
    pub fn new() -> CostByOrg {
        CostByOrg {
            attributes: None,
            id: None,
            r#type: None,
        }
    }
}



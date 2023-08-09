/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// RumApplicationList : RUM application list.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RumApplicationList {
    #[serde(rename = "attributes")]
    pub attributes: Box<crate::models::RumApplicationListAttributes>,
    /// RUM application ID.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub r#type: crate::models::RumApplicationListType,
}

impl RumApplicationList {
    /// RUM application list.
    pub fn new(attributes: crate::models::RumApplicationListAttributes, r#type: crate::models::RumApplicationListType) -> RumApplicationList {
        RumApplicationList {
            attributes: Box::new(attributes),
            id: None,
            r#type,
        }
    }
}



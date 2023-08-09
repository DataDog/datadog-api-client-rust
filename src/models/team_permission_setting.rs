/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// TeamPermissionSetting : Team permission setting



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TeamPermissionSetting {
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Box<crate::models::TeamPermissionSettingAttributes>>,
    /// The team permission setting's identifier
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "type")]
    pub r#type: crate::models::TeamPermissionSettingType,
}

impl TeamPermissionSetting {
    /// Team permission setting
    pub fn new(id: String, r#type: crate::models::TeamPermissionSettingType) -> TeamPermissionSetting {
        TeamPermissionSetting {
            attributes: None,
            id,
            r#type,
        }
    }
}



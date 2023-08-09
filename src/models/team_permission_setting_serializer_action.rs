/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// TeamPermissionSettingSerializerAction : The identifier for the action

/// The identifier for the action
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TeamPermissionSettingSerializerAction {
    #[serde(rename = "manage_membership")]
    MANAGE_MEMBERSHIP,
    #[serde(rename = "edit")]
    EDIT,

}

impl ToString for TeamPermissionSettingSerializerAction {
    fn to_string(&self) -> String {
        match self {
            Self::MANAGE_MEMBERSHIP => String::from("manage_membership"),
            Self::EDIT => String::from("edit"),
        }
    }
}

impl Default for TeamPermissionSettingSerializerAction {
    fn default() -> TeamPermissionSettingSerializerAction {
        Self::MANAGE_MEMBERSHIP
    }
}





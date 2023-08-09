/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// PermissionAttributes : Attributes of a permission.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PermissionAttributes {
    /// Creation time of the permission.
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// Description of the permission.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Displayed name for the permission.
    #[serde(rename = "display_name", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// Display type.
    #[serde(rename = "display_type", skip_serializing_if = "Option::is_none")]
    pub display_type: Option<String>,
    /// Name of the permission group.
    #[serde(rename = "group_name", skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    /// Name of the permission.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Whether or not the permission is restricted.
    #[serde(rename = "restricted", skip_serializing_if = "Option::is_none")]
    pub restricted: Option<bool>,
}

impl PermissionAttributes {
    /// Attributes of a permission.
    pub fn new() -> PermissionAttributes {
        PermissionAttributes {
            created: None,
            description: None,
            display_name: None,
            display_type: None,
            group_name: None,
            name: None,
            restricted: None,
        }
    }
}



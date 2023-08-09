/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// UserTeamUpdate : A user's relationship with a team



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UserTeamUpdate {
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Box<crate::models::UserTeamAttributes>>,
    #[serde(rename = "type")]
    pub r#type: crate::models::UserTeamType,
}

impl UserTeamUpdate {
    /// A user's relationship with a team
    pub fn new(r#type: crate::models::UserTeamType) -> UserTeamUpdate {
        UserTeamUpdate {
            attributes: None,
            r#type,
        }
    }
}



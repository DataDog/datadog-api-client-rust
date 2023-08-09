/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// IncidentTeamResponse : Response with an incident team payload.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IncidentTeamResponse {
    #[serde(rename = "data")]
    pub data: Box<crate::models::IncidentTeamResponseData>,
    /// Included objects from relationships.
    #[serde(rename = "included", skip_serializing_if = "Option::is_none")]
    pub included: Option<Vec<crate::models::IncidentTeamIncludedItems>>,
}

impl IncidentTeamResponse {
    /// Response with an incident team payload.
    pub fn new(data: crate::models::IncidentTeamResponseData) -> IncidentTeamResponse {
        IncidentTeamResponse {
            data: Box::new(data),
            included: None,
        }
    }
}



/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// IncidentTeamCreateAttributes : The incident team's attributes for a create request.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IncidentTeamCreateAttributes {
    /// Name of the incident team.
    #[serde(rename = "name")]
    pub name: String,
}

impl IncidentTeamCreateAttributes {
    /// The incident team's attributes for a create request.
    pub fn new(name: String) -> IncidentTeamCreateAttributes {
        IncidentTeamCreateAttributes {
            name,
        }
    }
}



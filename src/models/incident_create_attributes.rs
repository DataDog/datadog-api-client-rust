/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// IncidentCreateAttributes : The incident's attributes for a create request.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IncidentCreateAttributes {
    /// Required if `customer_impacted:\"true\"`. A summary of the impact customers experienced during the incident.
    #[serde(rename = "customer_impact_scope", skip_serializing_if = "Option::is_none")]
    pub customer_impact_scope: Option<String>,
    /// A flag indicating whether the incident caused customer impact.
    #[serde(rename = "customer_impacted")]
    pub customer_impacted: bool,
    /// A condensed view of the user-defined fields for which to create initial selections.
    #[serde(rename = "fields", skip_serializing_if = "Option::is_none")]
    pub fields: Option<::std::collections::HashMap<String, crate::models::IncidentFieldAttributes>>,
    /// An array of initial timeline cells to be placed at the beginning of the incident timeline.
    #[serde(rename = "initial_cells", skip_serializing_if = "Option::is_none")]
    pub initial_cells: Option<Vec<crate::models::IncidentTimelineCellCreateAttributes>>,
    /// Notification handles that will be notified of the incident at creation.
    #[serde(rename = "notification_handles", skip_serializing_if = "Option::is_none")]
    pub notification_handles: Option<Vec<crate::models::IncidentNotificationHandle>>,
    /// The title of the incident, which summarizes what happened.
    #[serde(rename = "title")]
    pub title: String,
}

impl IncidentCreateAttributes {
    /// The incident's attributes for a create request.
    pub fn new(customer_impacted: bool, title: String) -> IncidentCreateAttributes {
        IncidentCreateAttributes {
            customer_impact_scope: None,
            customer_impacted,
            fields: None,
            initial_cells: None,
            notification_handles: None,
            title,
        }
    }
}



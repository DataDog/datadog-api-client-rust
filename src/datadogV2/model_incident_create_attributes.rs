// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentCreateAttributes {
    /// Required if `customer_impacted:"true"`. A summary of the impact customers experienced during the incident.
    #[serde(rename = "customer_impact_scope", skip_serializing_if = "Option::is_none")]
    pub customer_impact_scope: String,
    /// A flag indicating whether the incident caused customer impact.
    #[serde(rename = "customer_impacted", skip_serializing_if = "Option::is_none")]
    pub customer_impacted: bool,
    /// A condensed view of the user-defined fields for which to create initial selections.
    #[serde(rename = "fields", skip_serializing_if = "Option::is_none")]
    pub fields: map[string]IncidentFieldAttributes,
    /// An array of initial timeline cells to be placed at the beginning of the incident timeline.
    #[serde(rename = "initial_cells", skip_serializing_if = "Option::is_none")]
    pub initial_cells: Vec<IncidentTimelineCellCreateAttributes>,
    /// Notification handles that will be notified of the incident at creation.
    #[serde(rename = "notification_handles", skip_serializing_if = "Option::is_none")]
    pub notification_handles: Vec<IncidentNotificationHandle>,
    /// The title of the incident, which summarizes what happened.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: String,
}


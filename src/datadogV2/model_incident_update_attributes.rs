// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentUpdateAttributes {
    /// Timestamp when customers were no longer impacted by the incident.
    #[serde(rename = "customer_impact_end", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub customer_impact_end: Option<Time>,
    /// A summary of the impact customers experienced during the incident.
    #[serde(rename = "customer_impact_scope", skip_serializing_if = "Option::is_none")]
    pub customer_impact_scope: String,
    /// Timestamp when customers began being impacted by the incident.
    #[serde(rename = "customer_impact_start", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub customer_impact_start: Option<Time>,
    /// A flag indicating whether the incident caused customer impact.
    #[serde(rename = "customer_impacted", skip_serializing_if = "Option::is_none")]
    pub customer_impacted: bool,
    /// Timestamp when the incident was detected.
    #[serde(rename = "detected", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub detected: Option<Time>,
    /// A condensed view of the user-defined fields for which to update selections.
    #[serde(rename = "fields", skip_serializing_if = "Option::is_none")]
    pub fields: map[string]IncidentFieldAttributes,
    /// Notification handles that will be notified of the incident during update.
    #[serde(rename = "notification_handles", skip_serializing_if = "Option::is_none")]
    pub notification_handles: Vec<IncidentNotificationHandle>,
    /// The title of the incident, which summarizes what happened.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: String,
}


// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentResponseAttributes {
    /// Timestamp when the incident was created.
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: String,
    /// Length of the incident's customer impact in seconds.
Equals the difference between `customer_impact_start` and `customer_impact_end`.
    #[serde(rename = "customer_impact_duration", skip_serializing_if = "Option::is_none")]
    pub customer_impact_duration: i64,
    /// Timestamp when customers were no longer impacted by the incident.
    #[serde(rename = "customer_impact_end", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub customer_impact_end: Option<Time>,
    /// A summary of the impact customers experienced during the incident.
    #[serde(rename = "customer_impact_scope", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub customer_impact_scope: Option<String>,
    /// Timestamp when customers began being impacted by the incident.
    #[serde(rename = "customer_impact_start", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub customer_impact_start: Option<Time>,
    /// A flag indicating whether the incident caused customer impact.
    #[serde(rename = "customer_impacted", skip_serializing_if = "Option::is_none")]
    pub customer_impacted: bool,
    /// Timestamp when the incident was detected.
    #[serde(rename = "detected", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub detected: Option<Time>,
    /// A condensed view of the user-defined fields attached to incidents.
    #[serde(rename = "fields", skip_serializing_if = "Option::is_none")]
    pub fields: map[string]IncidentFieldAttributes,
    /// Timestamp when the incident was last modified.
    #[serde(rename = "modified", skip_serializing_if = "Option::is_none")]
    pub modified: String,
    /// Notification handles that will be notified of the incident during update.
    #[serde(rename = "notification_handles", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub notification_handles: Vec<IncidentNotificationHandle>,
    /// The monotonically increasing integer ID for the incident.
    #[serde(rename = "public_id", skip_serializing_if = "Option::is_none")]
    pub public_id: i64,
    /// Timestamp when the incident's state was last changed from active or stable to resolved or completed.
    #[serde(rename = "resolved", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub resolved: Option<Time>,
    /// The amount of time in seconds to detect the incident.
Equals the difference between `customer_impact_start` and `detected`.
    #[serde(rename = "time_to_detect", skip_serializing_if = "Option::is_none")]
    pub time_to_detect: i64,
    /// The amount of time in seconds to call incident after detection. Equals the difference of `detected` and `created`.
    #[serde(rename = "time_to_internal_response", skip_serializing_if = "Option::is_none")]
    pub time_to_internal_response: i64,
    /// The amount of time in seconds to resolve customer impact after detecting the issue. Equals the difference between `customer_impact_end` and `detected`.
    #[serde(rename = "time_to_repair", skip_serializing_if = "Option::is_none")]
    pub time_to_repair: i64,
    /// The amount of time in seconds to resolve the incident after it was created. Equals the difference between `created` and `resolved`.
    #[serde(rename = "time_to_resolve", skip_serializing_if = "Option::is_none")]
    pub time_to_resolve: i64,
    /// The title of the incident, which summarizes what happened.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: String,
}


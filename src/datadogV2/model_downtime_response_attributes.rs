// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DowntimeResponseAttributes {
    /// Time that the downtime was canceled.
    #[serde(rename = "canceled", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub canceled: Option<Time>,
    /// Creation time of the downtime.
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: String,
    /// The timezone in which to display the downtime's start and end times in Datadog applications. This is not used
as an offset for scheduling.
    #[serde(rename = "display_timezone", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub display_timezone: Option<String>,
    /// A message to include with notifications for this downtime. Email notifications can be sent to specific users
by using the same `@username` notation as events.
    #[serde(rename = "message", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub message: Option<String>,
    /// Time that the downtime was last modified.
    #[serde(rename = "modified", skip_serializing_if = "Option::is_none")]
    pub modified: String,
    /// Monitor identifier for the downtime.
    #[serde(rename = "monitor_identifier", skip_serializing_if = "Option::is_none")]
    pub monitor_identifier: DowntimeMonitorIdentifier,
    /// If the first recovery notification during a downtime should be muted.
    #[serde(rename = "mute_first_recovery_notification", skip_serializing_if = "Option::is_none")]
    pub mute_first_recovery_notification: bool,
    /// States that will trigger a monitor notification when the `notify_end_types` action occurs.
    #[serde(rename = "notify_end_states", skip_serializing_if = "Option::is_none")]
    pub notify_end_states: Vec<DowntimeNotifyEndStateTypes>,
    /// Actions that will trigger a monitor notification if the downtime is in the `notify_end_types` state.
    #[serde(rename = "notify_end_types", skip_serializing_if = "Option::is_none")]
    pub notify_end_types: Vec<DowntimeNotifyEndStateActions>,
    /// The schedule that defines when the monitor starts, stops, and recurs. There are two types of schedules:
one-time and recurring. Recurring schedules may have up to five RRULE-based recurrences. If no schedules are
provided, the downtime will begin immediately and never end.
    #[serde(rename = "schedule", skip_serializing_if = "Option::is_none")]
    pub schedule: DowntimeScheduleResponse,
    /// The scope to which the downtime applies. Must follow the [common search syntax](https://docs.datadoghq.com/logs/explorer/search_syntax/).
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: String,
    /// The current status of the downtime.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: DowntimeStatus,
}


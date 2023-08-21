// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DowntimeCreateRequestAttributes {
    /// The timezone in which to display the downtime's start and end times in Datadog applications. This is not used
as an offset for scheduling.
    #[serde(rename = "display_timezone", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub display_timezone: Option<String>,
    /// A message to include with notifications for this downtime. Email notifications can be sent to specific users
by using the same `@username` notation as events.
    #[serde(rename = "message", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub message: Option<String>,
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
    /// Schedule for the downtime.
    #[serde(rename = "schedule", skip_serializing_if = "Option::is_none")]
    pub schedule: DowntimeScheduleCreateRequest,
    /// The scope to which the downtime applies. Must follow the [common search syntax](https://docs.datadoghq.com/logs/explorer/search_syntax/).
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: String,
}


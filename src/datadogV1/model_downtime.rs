// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Downtime {
    /// If a scheduled downtime currently exists.
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: bool,
    /// The downtime object definition of the active child for the original parent recurring downtime. This
field will only exist on recurring downtimes.
    #[serde(rename = "active_child", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub active_child: NullableDowntimeChild,
    /// If a scheduled downtime is canceled.
    #[serde(rename = "canceled", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub canceled: Option<Int64>,
    /// User ID of the downtime creator.
    #[serde(rename = "creator_id", skip_serializing_if = "Option::is_none")]
    pub creator_id: i32,
    /// If a downtime has been disabled.
    #[serde(rename = "disabled", skip_serializing_if = "Option::is_none")]
    pub disabled: bool,
    /// `0` for a downtime applied on `*` or all,
`1` when the downtime is only scoped to hosts,
or `2` when the downtime is scoped to anything but hosts.
    #[serde(rename = "downtime_type", skip_serializing_if = "Option::is_none")]
    pub downtime_type: i32,
    /// POSIX timestamp to end the downtime. If not provided,
the downtime is in effect indefinitely until you cancel it.
    #[serde(rename = "end", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub end: Option<Int64>,
    /// The downtime ID.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: i64,
    /// A message to include with notifications for this downtime.
Email notifications can be sent to specific users by using the same `@username` notation as events.
    #[serde(rename = "message", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub message: Option<String>,
    /// A single monitor to which the downtime applies.
If not provided, the downtime applies to all monitors.
    #[serde(rename = "monitor_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub monitor_id: Option<Int64>,
    /// A comma-separated list of monitor tags. For example, tags that are applied directly to monitors,
not tags that are used in monitor queries (which are filtered by the scope parameter), to which the downtime applies.
The resulting downtime applies to monitors that match ALL provided monitor tags.
For example, `service:postgres` **AND** `team:frontend`.
    #[serde(rename = "monitor_tags", skip_serializing_if = "Option::is_none")]
    pub monitor_tags: Vec<String>,
    /// If the first recovery notification during a downtime should be muted.
    #[serde(rename = "mute_first_recovery_notification", skip_serializing_if = "Option::is_none")]
    pub mute_first_recovery_notification: bool,
    /// States for which `notify_end_types` sends out notifications for.
    #[serde(rename = "notify_end_states", skip_serializing_if = "Option::is_none")]
    pub notify_end_states: Vec<NotifyEndState>,
    /// If set, notifies if a monitor is in an alert-worthy state (`ALERT`, `WARNING`, or `NO DATA`)
when this downtime expires or is canceled. Applied to monitors that change states during
the downtime (such as from `OK` to `ALERT`, `WARNING`, or `NO DATA`), and to monitors that
already have an alert-worthy state when downtime begins.
    #[serde(rename = "notify_end_types", skip_serializing_if = "Option::is_none")]
    pub notify_end_types: Vec<NotifyEndType>,
    /// ID of the parent Downtime.
    #[serde(rename = "parent_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub parent_id: Option<Int64>,
    /// An object defining the recurrence of the downtime.
    #[serde(rename = "recurrence", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub recurrence: NullableDowntimeRecurrence,
    /// The scope(s) to which the downtime applies and must be in `key:value` format. For example, `host:app2`.
Provide multiple scopes as a comma-separated list like `env:dev,env:prod`.
The resulting downtime applies to sources that matches ALL provided scopes (`env:dev` **AND** `env:prod`).
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: Vec<String>,
    /// POSIX timestamp to start the downtime.
If not provided, the downtime starts the moment it is created.
    #[serde(rename = "start", skip_serializing_if = "Option::is_none")]
    pub start: i64,
    /// The timezone in which to display the downtime's start and end times in Datadog applications.
    #[serde(rename = "timezone", skip_serializing_if = "Option::is_none")]
    pub timezone: String,
    /// ID of the last user that updated the downtime.
    #[serde(rename = "updater_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub updater_id: Option<Int32>,
}


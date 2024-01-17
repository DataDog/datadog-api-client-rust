// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The downtime object definition of the active child for the original parent recurring downtime. This
/// field will only exist on recurring downtimes.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DowntimeChild {
    /// If a scheduled downtime currently exists.
    #[serde(rename = "active")]
    pub active: Option<bool>,
    /// If a scheduled downtime is canceled.
    #[serde(
        rename = "canceled",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub canceled: Option<Option<i64>>,
    /// User ID of the downtime creator.
    #[serde(rename = "creator_id")]
    pub creator_id: Option<i32>,
    /// If a downtime has been disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
    /// `0` for a downtime applied on `*` or all,
    /// `1` when the downtime is only scoped to hosts,
    /// or `2` when the downtime is scoped to anything but hosts.
    #[serde(rename = "downtime_type")]
    pub downtime_type: Option<i32>,
    /// POSIX timestamp to end the downtime. If not provided,
    /// the downtime is in effect indefinitely until you cancel it.
    #[serde(rename = "end", default, with = "::serde_with::rust::double_option")]
    pub end: Option<Option<i64>>,
    /// The downtime ID.
    #[serde(rename = "id")]
    pub id: Option<i64>,
    /// A message to include with notifications for this downtime.
    /// Email notifications can be sent to specific users by using the same `@username` notation as events.
    #[serde(
        rename = "message",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub message: Option<Option<String>>,
    /// A single monitor to which the downtime applies.
    /// If not provided, the downtime applies to all monitors.
    #[serde(
        rename = "monitor_id",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub monitor_id: Option<Option<i64>>,
    /// A comma-separated list of monitor tags. For example, tags that are applied directly to monitors,
    /// not tags that are used in monitor queries (which are filtered by the scope parameter), to which the downtime applies.
    /// The resulting downtime applies to monitors that match ALL provided monitor tags.
    /// For example, `service:postgres` **AND** `team:frontend`.
    #[serde(rename = "monitor_tags")]
    pub monitor_tags: Option<Vec<String>>,
    /// If the first recovery notification during a downtime should be muted.
    #[serde(rename = "mute_first_recovery_notification")]
    pub mute_first_recovery_notification: Option<bool>,
    /// States for which `notify_end_types` sends out notifications for.
    #[serde(rename = "notify_end_states")]
    pub notify_end_states: Option<Vec<crate::datadogV1::model::NotifyEndState>>,
    /// If set, notifies if a monitor is in an alert-worthy state (`ALERT`, `WARNING`, or `NO DATA`)
    /// when this downtime expires or is canceled. Applied to monitors that change states during
    /// the downtime (such as from `OK` to `ALERT`, `WARNING`, or `NO DATA`), and to monitors that
    /// already have an alert-worthy state when downtime begins.
    #[serde(rename = "notify_end_types")]
    pub notify_end_types: Option<Vec<crate::datadogV1::model::NotifyEndType>>,
    /// ID of the parent Downtime.
    #[serde(
        rename = "parent_id",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub parent_id: Option<Option<i64>>,
    /// An object defining the recurrence of the downtime.
    #[serde(
        rename = "recurrence",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub recurrence: Option<Option<Box<crate::datadogV1::model::DowntimeRecurrence>>>,
    /// The scope(s) to which the downtime applies and must be in `key:value` format. For example, `host:app2`.
    /// Provide multiple scopes as a comma-separated list like `env:dev,env:prod`.
    /// The resulting downtime applies to sources that matches ALL provided scopes (`env:dev` **AND** `env:prod`).
    #[serde(rename = "scope")]
    pub scope: Option<Vec<String>>,
    /// POSIX timestamp to start the downtime.
    /// If not provided, the downtime starts the moment it is created.
    #[serde(rename = "start")]
    pub start: Option<i64>,
    /// The timezone in which to display the downtime's start and end times in Datadog applications.
    #[serde(rename = "timezone")]
    pub timezone: Option<String>,
    /// ID of the last user that updated the downtime.
    #[serde(
        rename = "updater_id",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub updater_id: Option<Option<i32>>,
}

impl DowntimeChild {
    pub fn new() -> DowntimeChild {
        DowntimeChild {
            active: None,
            canceled: None,
            creator_id: None,
            disabled: None,
            downtime_type: None,
            end: None,
            id: None,
            message: None,
            monitor_id: None,
            monitor_tags: None,
            mute_first_recovery_notification: None,
            notify_end_states: None,
            notify_end_types: None,
            parent_id: None,
            recurrence: None,
            scope: None,
            start: None,
            timezone: None,
            updater_id: None,
        }
    }
}
impl Default for DowntimeChild {
    fn default() -> Self {
        Self::new()
    }
}

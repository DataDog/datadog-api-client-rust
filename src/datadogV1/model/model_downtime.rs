// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Downtiming gives you greater control over monitor notifications by
/// allowing you to globally exclude scopes from alerting.
/// Downtime settings, which can be scheduled with start and end times,
/// prevent all alerting related to specified Datadog tags.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Downtime {
    /// If a scheduled downtime currently exists.
    #[serde(rename = "active")]
    pub active: Option<bool>,
    /// The downtime object definition of the active child for the original parent recurring downtime. This
    /// field will only exist on recurring downtimes.
    #[serde(
        rename = "active_child",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub active_child: Option<Option<crate::datadogV1::model::DowntimeChild>>,
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
    pub recurrence: Option<Option<crate::datadogV1::model::DowntimeRecurrence>>,
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

impl Downtime {
    pub fn new() -> Downtime {
        Downtime {
            active: None,
            active_child: None,
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

    pub fn with_active(&mut self, value: bool) -> &mut Self {
        self.active = Some(value);
        self
    }

    pub fn with_active_child(
        &mut self,
        value: Option<crate::datadogV1::model::DowntimeChild>,
    ) -> &mut Self {
        self.active_child = Some(value);
        self
    }

    pub fn with_canceled(&mut self, value: Option<i64>) -> &mut Self {
        self.canceled = Some(value);
        self
    }

    pub fn with_creator_id(&mut self, value: i32) -> &mut Self {
        self.creator_id = Some(value);
        self
    }

    pub fn with_disabled(&mut self, value: bool) -> &mut Self {
        self.disabled = Some(value);
        self
    }

    pub fn with_downtime_type(&mut self, value: i32) -> &mut Self {
        self.downtime_type = Some(value);
        self
    }

    pub fn with_end(&mut self, value: Option<i64>) -> &mut Self {
        self.end = Some(value);
        self
    }

    pub fn with_id(&mut self, value: i64) -> &mut Self {
        self.id = Some(value);
        self
    }

    pub fn with_message(&mut self, value: Option<String>) -> &mut Self {
        self.message = Some(value);
        self
    }

    pub fn with_monitor_id(&mut self, value: Option<i64>) -> &mut Self {
        self.monitor_id = Some(value);
        self
    }

    pub fn with_monitor_tags(&mut self, value: Vec<String>) -> &mut Self {
        self.monitor_tags = Some(value);
        self
    }

    pub fn with_mute_first_recovery_notification(&mut self, value: bool) -> &mut Self {
        self.mute_first_recovery_notification = Some(value);
        self
    }

    pub fn with_notify_end_states(
        &mut self,
        value: Vec<crate::datadogV1::model::NotifyEndState>,
    ) -> &mut Self {
        self.notify_end_states = Some(value);
        self
    }

    pub fn with_notify_end_types(
        &mut self,
        value: Vec<crate::datadogV1::model::NotifyEndType>,
    ) -> &mut Self {
        self.notify_end_types = Some(value);
        self
    }

    pub fn with_parent_id(&mut self, value: Option<i64>) -> &mut Self {
        self.parent_id = Some(value);
        self
    }

    pub fn with_recurrence(
        &mut self,
        value: Option<crate::datadogV1::model::DowntimeRecurrence>,
    ) -> &mut Self {
        self.recurrence = Some(value);
        self
    }

    pub fn with_scope(&mut self, value: Vec<String>) -> &mut Self {
        self.scope = Some(value);
        self
    }

    pub fn with_start(&mut self, value: i64) -> &mut Self {
        self.start = Some(value);
        self
    }

    pub fn with_timezone(&mut self, value: String) -> &mut Self {
        self.timezone = Some(value);
        self
    }

    pub fn with_updater_id(&mut self, value: Option<i32>) -> &mut Self {
        self.updater_id = Some(value);
        self
    }
}
impl Default for Downtime {
    fn default() -> Self {
        Self::new()
    }
}

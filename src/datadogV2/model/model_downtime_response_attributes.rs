// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Downtime details.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DowntimeResponseAttributes {
    /// Time that the downtime was canceled.
    #[serde(
        rename = "canceled",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub canceled: Option<Option<String>>,
    /// Creation time of the downtime.
    #[serde(rename = "created")]
    pub created: Option<String>,
    /// The timezone in which to display the downtime's start and end times in Datadog applications. This is not used
    /// as an offset for scheduling.
    #[serde(
        rename = "display_timezone",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub display_timezone: Option<Option<String>>,
    /// A message to include with notifications for this downtime. Email notifications can be sent to specific users
    /// by using the same `@username` notation as events.
    #[serde(
        rename = "message",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub message: Option<Option<String>>,
    /// Time that the downtime was last modified.
    #[serde(rename = "modified")]
    pub modified: Option<String>,
    /// Monitor identifier for the downtime.
    #[serde(rename = "monitor_identifier")]
    pub monitor_identifier: Option<crate::datadogV2::model::DowntimeMonitorIdentifier>,
    /// If the first recovery notification during a downtime should be muted.
    #[serde(rename = "mute_first_recovery_notification")]
    pub mute_first_recovery_notification: Option<bool>,
    /// States that will trigger a monitor notification when the `notify_end_types` action occurs.
    #[serde(rename = "notify_end_states")]
    pub notify_end_states: Option<Vec<crate::datadogV2::model::DowntimeNotifyEndStateTypes>>,
    /// Actions that will trigger a monitor notification if the downtime is in the `notify_end_types` state.
    #[serde(rename = "notify_end_types")]
    pub notify_end_types: Option<Vec<crate::datadogV2::model::DowntimeNotifyEndStateActions>>,
    /// The schedule that defines when the monitor starts, stops, and recurs. There are two types of schedules:
    /// one-time and recurring. Recurring schedules may have up to five RRULE-based recurrences. If no schedules are
    /// provided, the downtime will begin immediately and never end.
    #[serde(rename = "schedule")]
    pub schedule: Option<crate::datadogV2::model::DowntimeScheduleResponse>,
    /// The scope to which the downtime applies. Must follow the [common search syntax](<https://docs.datadoghq.com/logs/explorer/search_syntax/>).
    #[serde(rename = "scope")]
    pub scope: Option<String>,
    /// The current status of the downtime.
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV2::model::DowntimeStatus>,
}

impl DowntimeResponseAttributes {
    pub fn new() -> DowntimeResponseAttributes {
        DowntimeResponseAttributes {
            canceled: None,
            created: None,
            display_timezone: None,
            message: None,
            modified: None,
            monitor_identifier: None,
            mute_first_recovery_notification: None,
            notify_end_states: None,
            notify_end_types: None,
            schedule: None,
            scope: None,
            status: None,
        }
    }

    pub fn canceled(&mut self, value: Option<String>) -> &mut Self {
        self.canceled = Some(value);
        self
    }

    pub fn created(&mut self, value: String) -> &mut Self {
        self.created = Some(value);
        self
    }

    pub fn display_timezone(&mut self, value: Option<String>) -> &mut Self {
        self.display_timezone = Some(value);
        self
    }

    pub fn message(&mut self, value: Option<String>) -> &mut Self {
        self.message = Some(value);
        self
    }

    pub fn modified(&mut self, value: String) -> &mut Self {
        self.modified = Some(value);
        self
    }

    pub fn monitor_identifier(
        &mut self,
        value: crate::datadogV2::model::DowntimeMonitorIdentifier,
    ) -> &mut Self {
        self.monitor_identifier = Some(value);
        self
    }

    pub fn mute_first_recovery_notification(&mut self, value: bool) -> &mut Self {
        self.mute_first_recovery_notification = Some(value);
        self
    }

    pub fn notify_end_states(
        &mut self,
        value: Vec<crate::datadogV2::model::DowntimeNotifyEndStateTypes>,
    ) -> &mut Self {
        self.notify_end_states = Some(value);
        self
    }

    pub fn notify_end_types(
        &mut self,
        value: Vec<crate::datadogV2::model::DowntimeNotifyEndStateActions>,
    ) -> &mut Self {
        self.notify_end_types = Some(value);
        self
    }

    pub fn schedule(
        &mut self,
        value: crate::datadogV2::model::DowntimeScheduleResponse,
    ) -> &mut Self {
        self.schedule = Some(value);
        self
    }

    pub fn scope(&mut self, value: String) -> &mut Self {
        self.scope = Some(value);
        self
    }

    pub fn status(&mut self, value: crate::datadogV2::model::DowntimeStatus) -> &mut Self {
        self.status = Some(value);
        self
    }
}

impl Default for DowntimeResponseAttributes {
    fn default() -> Self {
        Self::new()
    }
}

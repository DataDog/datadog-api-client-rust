// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Attributes of the downtime to update.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DowntimeUpdateRequestAttributes {
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
    /// Schedule for the downtime.
    #[serde(rename = "schedule")]
    pub schedule: Option<crate::datadogV2::model::DowntimeScheduleUpdateRequest>,
    /// The scope to which the downtime applies. Must follow the [common search syntax](<https://docs.datadoghq.com/logs/explorer/search_syntax/>).
    #[serde(rename = "scope")]
    pub scope: Option<String>,
}

impl DowntimeUpdateRequestAttributes {
    pub fn new() -> DowntimeUpdateRequestAttributes {
        DowntimeUpdateRequestAttributes {
            display_timezone: None,
            message: None,
            monitor_identifier: None,
            mute_first_recovery_notification: None,
            notify_end_states: None,
            notify_end_types: None,
            schedule: None,
            scope: None,
        }
    }

    pub fn display_timezone(&mut self, value: Option<String>) -> &mut Self {
        self.display_timezone = Some(value);
        self
    }

    pub fn message(&mut self, value: Option<String>) -> &mut Self {
        self.message = Some(value);
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
        value: crate::datadogV2::model::DowntimeScheduleUpdateRequest,
    ) -> &mut Self {
        self.schedule = Some(value);
        self
    }

    pub fn scope(&mut self, value: String) -> &mut Self {
        self.scope = Some(value);
        self
    }
}

impl Default for DowntimeUpdateRequestAttributes {
    fn default() -> Self {
        Self::new()
    }
}

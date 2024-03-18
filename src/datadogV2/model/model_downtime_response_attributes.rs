// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Downtime details.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DowntimeResponseAttributes {
    /// Time that the downtime was canceled.
    #[serde(
        rename = "canceled",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub canceled: Option<Option<DateTime<Utc>>>,
    /// Creation time of the downtime.
    #[serde(rename = "created")]
    pub created: Option<DateTime<Utc>>,
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
    pub modified: Option<DateTime<Utc>>,
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
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
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
            _unparsed: false,
        }
    }

    pub fn canceled(mut self, value: Option<DateTime<Utc>>) -> Self {
        self.canceled = Some(value);
        self
    }

    pub fn created(mut self, value: DateTime<Utc>) -> Self {
        self.created = Some(value);
        self
    }

    pub fn display_timezone(mut self, value: Option<String>) -> Self {
        self.display_timezone = Some(value);
        self
    }

    pub fn message(mut self, value: Option<String>) -> Self {
        self.message = Some(value);
        self
    }

    pub fn modified(mut self, value: DateTime<Utc>) -> Self {
        self.modified = Some(value);
        self
    }

    pub fn monitor_identifier(
        mut self,
        value: crate::datadogV2::model::DowntimeMonitorIdentifier,
    ) -> Self {
        self.monitor_identifier = Some(value);
        self
    }

    pub fn mute_first_recovery_notification(mut self, value: bool) -> Self {
        self.mute_first_recovery_notification = Some(value);
        self
    }

    pub fn notify_end_states(
        mut self,
        value: Vec<crate::datadogV2::model::DowntimeNotifyEndStateTypes>,
    ) -> Self {
        self.notify_end_states = Some(value);
        self
    }

    pub fn notify_end_types(
        mut self,
        value: Vec<crate::datadogV2::model::DowntimeNotifyEndStateActions>,
    ) -> Self {
        self.notify_end_types = Some(value);
        self
    }

    pub fn schedule(mut self, value: crate::datadogV2::model::DowntimeScheduleResponse) -> Self {
        self.schedule = Some(value);
        self
    }

    pub fn scope(mut self, value: String) -> Self {
        self.scope = Some(value);
        self
    }

    pub fn status(mut self, value: crate::datadogV2::model::DowntimeStatus) -> Self {
        self.status = Some(value);
        self
    }
}

impl Default for DowntimeResponseAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for DowntimeResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DowntimeResponseAttributesVisitor;
        impl<'a> Visitor<'a> for DowntimeResponseAttributesVisitor {
            type Value = DowntimeResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut canceled: Option<Option<DateTime<Utc>>> = None;
                let mut created: Option<DateTime<Utc>> = None;
                let mut display_timezone: Option<Option<String>> = None;
                let mut message: Option<Option<String>> = None;
                let mut modified: Option<DateTime<Utc>> = None;
                let mut monitor_identifier: Option<
                    crate::datadogV2::model::DowntimeMonitorIdentifier,
                > = None;
                let mut mute_first_recovery_notification: Option<bool> = None;
                let mut notify_end_states: Option<
                    Vec<crate::datadogV2::model::DowntimeNotifyEndStateTypes>,
                > = None;
                let mut notify_end_types: Option<
                    Vec<crate::datadogV2::model::DowntimeNotifyEndStateActions>,
                > = None;
                let mut schedule: Option<crate::datadogV2::model::DowntimeScheduleResponse> = None;
                let mut scope: Option<String> = None;
                let mut status: Option<crate::datadogV2::model::DowntimeStatus> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "canceled" => {
                            canceled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created" => {
                            if v.is_null() {
                                continue;
                            }
                            created = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "display_timezone" => {
                            display_timezone =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "message" => {
                            message = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified" => {
                            if v.is_null() {
                                continue;
                            }
                            modified = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "monitor_identifier" => {
                            if v.is_null() {
                                continue;
                            }
                            monitor_identifier =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _monitor_identifier) = monitor_identifier {
                                match _monitor_identifier {
                                    crate::datadogV2::model::DowntimeMonitorIdentifier::UnparsedObject(_monitor_identifier) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "mute_first_recovery_notification" => {
                            if v.is_null() {
                                continue;
                            }
                            mute_first_recovery_notification =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "notify_end_states" => {
                            if v.is_null() {
                                continue;
                            }
                            notify_end_states =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "notify_end_types" => {
                            if v.is_null() {
                                continue;
                            }
                            notify_end_types =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "schedule" => {
                            if v.is_null() {
                                continue;
                            }
                            schedule = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _schedule) = schedule {
                                match _schedule {
                                    crate::datadogV2::model::DowntimeScheduleResponse::UnparsedObject(_schedule) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "scope" => {
                            if v.is_null() {
                                continue;
                            }
                            scope = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::DowntimeStatus::UnparsedObject(
                                        _status,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }

                let content = DowntimeResponseAttributes {
                    canceled,
                    created,
                    display_timezone,
                    message,
                    modified,
                    monitor_identifier,
                    mute_first_recovery_notification,
                    notify_end_states,
                    notify_end_types,
                    schedule,
                    scope,
                    status,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DowntimeResponseAttributesVisitor)
    }
}

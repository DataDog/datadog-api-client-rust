// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Downtime details.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DowntimeCreateRequestAttributes {
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
    pub monitor_identifier: crate::datadogV2::model::DowntimeMonitorIdentifier,
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
    pub schedule: Option<crate::datadogV2::model::DowntimeScheduleCreateRequest>,
    /// The scope to which the downtime applies. Must follow the [common search syntax](<https://docs.datadoghq.com/logs/explorer/search_syntax/>).
    #[serde(rename = "scope")]
    pub scope: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DowntimeCreateRequestAttributes {
    pub fn new(
        monitor_identifier: crate::datadogV2::model::DowntimeMonitorIdentifier,
        scope: String,
    ) -> DowntimeCreateRequestAttributes {
        DowntimeCreateRequestAttributes {
            display_timezone: None,
            message: None,
            monitor_identifier,
            mute_first_recovery_notification: None,
            notify_end_states: None,
            notify_end_types: None,
            schedule: None,
            scope,
            _unparsed: false,
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
        value: crate::datadogV2::model::DowntimeScheduleCreateRequest,
    ) -> &mut Self {
        self.schedule = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for DowntimeCreateRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DowntimeCreateRequestAttributesVisitor;
        impl<'a> Visitor<'a> for DowntimeCreateRequestAttributesVisitor {
            type Value = DowntimeCreateRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut display_timezone: Option<Option<String>> = None;
                let mut message: Option<Option<String>> = None;
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
                let mut schedule: Option<crate::datadogV2::model::DowntimeScheduleCreateRequest> =
                    None;
                let mut scope: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "display_timezone" => {
                            display_timezone =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "message" => {
                            message = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "monitor_identifier" => {
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
                                    crate::datadogV2::model::DowntimeScheduleCreateRequest::UnparsedObject(_schedule) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "scope" => {
                            scope = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let monitor_identifier = monitor_identifier
                    .ok_or_else(|| M::Error::missing_field("monitor_identifier"))?;
                let scope = scope.ok_or_else(|| M::Error::missing_field("scope"))?;

                let content = DowntimeCreateRequestAttributes {
                    display_timezone,
                    message,
                    monitor_identifier,
                    mute_first_recovery_notification,
                    notify_end_states,
                    notify_end_types,
                    schedule,
                    scope,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DowntimeCreateRequestAttributesVisitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of the downtime to update.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
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
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
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
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn display_timezone(mut self, value: Option<String>) -> Self {
        self.display_timezone = Some(value);
        self
    }

    pub fn message(mut self, value: Option<String>) -> Self {
        self.message = Some(value);
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

    pub fn schedule(
        mut self,
        value: crate::datadogV2::model::DowntimeScheduleUpdateRequest,
    ) -> Self {
        self.schedule = Some(value);
        self
    }

    pub fn scope(mut self, value: String) -> Self {
        self.scope = Some(value);
        self
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl Default for DowntimeUpdateRequestAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for DowntimeUpdateRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DowntimeUpdateRequestAttributesVisitor;
        impl<'a> Visitor<'a> for DowntimeUpdateRequestAttributesVisitor {
            type Value = DowntimeUpdateRequestAttributes;

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
                let mut schedule: Option<crate::datadogV2::model::DowntimeScheduleUpdateRequest> =
                    None;
                let mut scope: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
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
                                    crate::datadogV2::model::DowntimeScheduleUpdateRequest::UnparsedObject(_schedule) => {
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
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = DowntimeUpdateRequestAttributes {
                    display_timezone,
                    message,
                    monitor_identifier,
                    mute_first_recovery_notification,
                    notify_end_states,
                    notify_end_types,
                    schedule,
                    scope,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DowntimeUpdateRequestAttributesVisitor)
    }
}

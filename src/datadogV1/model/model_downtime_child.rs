// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The downtime object definition of the active child for the original parent recurring downtime. This
/// field will only exist on recurring downtimes.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
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
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
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
            _unparsed: false,
        }
    }

    pub fn active(mut self, value: bool) -> Self {
        self.active = Some(value);
        self
    }

    pub fn canceled(mut self, value: Option<i64>) -> Self {
        self.canceled = Some(value);
        self
    }

    pub fn creator_id(mut self, value: i32) -> Self {
        self.creator_id = Some(value);
        self
    }

    pub fn disabled(mut self, value: bool) -> Self {
        self.disabled = Some(value);
        self
    }

    pub fn downtime_type(mut self, value: i32) -> Self {
        self.downtime_type = Some(value);
        self
    }

    pub fn end(mut self, value: Option<i64>) -> Self {
        self.end = Some(value);
        self
    }

    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn message(mut self, value: Option<String>) -> Self {
        self.message = Some(value);
        self
    }

    pub fn monitor_id(mut self, value: Option<i64>) -> Self {
        self.monitor_id = Some(value);
        self
    }

    pub fn monitor_tags(mut self, value: Vec<String>) -> Self {
        self.monitor_tags = Some(value);
        self
    }

    pub fn mute_first_recovery_notification(mut self, value: bool) -> Self {
        self.mute_first_recovery_notification = Some(value);
        self
    }

    pub fn notify_end_states(
        mut self,
        value: Vec<crate::datadogV1::model::NotifyEndState>,
    ) -> Self {
        self.notify_end_states = Some(value);
        self
    }

    pub fn notify_end_types(mut self, value: Vec<crate::datadogV1::model::NotifyEndType>) -> Self {
        self.notify_end_types = Some(value);
        self
    }

    pub fn parent_id(mut self, value: Option<i64>) -> Self {
        self.parent_id = Some(value);
        self
    }

    pub fn recurrence(
        mut self,
        value: Option<crate::datadogV1::model::DowntimeRecurrence>,
    ) -> Self {
        self.recurrence = Some(value);
        self
    }

    pub fn scope(mut self, value: Vec<String>) -> Self {
        self.scope = Some(value);
        self
    }

    pub fn start(mut self, value: i64) -> Self {
        self.start = Some(value);
        self
    }

    pub fn timezone(mut self, value: String) -> Self {
        self.timezone = Some(value);
        self
    }

    pub fn updater_id(mut self, value: Option<i32>) -> Self {
        self.updater_id = Some(value);
        self
    }
}

impl Default for DowntimeChild {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for DowntimeChild {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DowntimeChildVisitor;
        impl<'a> Visitor<'a> for DowntimeChildVisitor {
            type Value = DowntimeChild;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut active: Option<bool> = None;
                let mut canceled: Option<Option<i64>> = None;
                let mut creator_id: Option<i32> = None;
                let mut disabled: Option<bool> = None;
                let mut downtime_type: Option<i32> = None;
                let mut end: Option<Option<i64>> = None;
                let mut id: Option<i64> = None;
                let mut message: Option<Option<String>> = None;
                let mut monitor_id: Option<Option<i64>> = None;
                let mut monitor_tags: Option<Vec<String>> = None;
                let mut mute_first_recovery_notification: Option<bool> = None;
                let mut notify_end_states: Option<Vec<crate::datadogV1::model::NotifyEndState>> =
                    None;
                let mut notify_end_types: Option<Vec<crate::datadogV1::model::NotifyEndType>> =
                    None;
                let mut parent_id: Option<Option<i64>> = None;
                let mut recurrence: Option<Option<crate::datadogV1::model::DowntimeRecurrence>> =
                    None;
                let mut scope: Option<Vec<String>> = None;
                let mut start: Option<i64> = None;
                let mut timezone: Option<String> = None;
                let mut updater_id: Option<Option<i32>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "active" => {
                            if v.is_null() {
                                continue;
                            }
                            active = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "canceled" => {
                            canceled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "creator_id" => {
                            if v.is_null() {
                                continue;
                            }
                            creator_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "disabled" => {
                            if v.is_null() {
                                continue;
                            }
                            disabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "downtime_type" => {
                            if v.is_null() {
                                continue;
                            }
                            downtime_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "end" => {
                            end = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "message" => {
                            message = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "monitor_id" => {
                            monitor_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "monitor_tags" => {
                            if v.is_null() {
                                continue;
                            }
                            monitor_tags =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "parent_id" => {
                            parent_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "recurrence" => {
                            recurrence = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "scope" => {
                            if v.is_null() {
                                continue;
                            }
                            scope = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "start" => {
                            if v.is_null() {
                                continue;
                            }
                            start = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timezone" => {
                            if v.is_null() {
                                continue;
                            }
                            timezone = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updater_id" => {
                            updater_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = DowntimeChild {
                    active,
                    canceled,
                    creator_id,
                    disabled,
                    downtime_type,
                    end,
                    id,
                    message,
                    monitor_id,
                    monitor_tags,
                    mute_first_recovery_notification,
                    notify_end_states,
                    notify_end_types,
                    parent_id,
                    recurrence,
                    scope,
                    start,
                    timezone,
                    updater_id,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DowntimeChildVisitor)
    }
}

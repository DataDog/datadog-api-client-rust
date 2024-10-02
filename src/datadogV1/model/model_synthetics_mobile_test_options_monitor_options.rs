// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object containing the options for a mobile Synthetic test as a monitor
/// (for example, renotification).
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsMobileTestOptionsMonitorOptions {
    /// Message to include in the escalation notification.
    #[serde(rename = "escalation_message")]
    pub escalation_message: Option<String>,
    /// The definition of `SyntheticsMobileTestOptionsMonitorOptionsNotificationPresetName` object.
    #[serde(rename = "notification_preset_name")]
    pub notification_preset_name: Option<
        crate::datadogV1::model::SyntheticsMobileTestOptionsMonitorOptionsNotificationPresetName,
    >,
    /// Time interval before renotifying if the test is still failing
    /// (in minutes).
    #[serde(rename = "renotify_interval")]
    pub renotify_interval: Option<i64>,
    /// The `SyntheticsMobileTestOptionsMonitorOptions` `renotify_occurrences`.
    #[serde(rename = "renotify_occurrences")]
    pub renotify_occurrences: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsMobileTestOptionsMonitorOptions {
    pub fn new() -> SyntheticsMobileTestOptionsMonitorOptions {
        SyntheticsMobileTestOptionsMonitorOptions {
            escalation_message: None,
            notification_preset_name: None,
            renotify_interval: None,
            renotify_occurrences: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn escalation_message(mut self, value: String) -> Self {
        self.escalation_message = Some(value);
        self
    }

    pub fn notification_preset_name(
        mut self,
        value: crate::datadogV1::model::SyntheticsMobileTestOptionsMonitorOptionsNotificationPresetName,
    ) -> Self {
        self.notification_preset_name = Some(value);
        self
    }

    pub fn renotify_interval(mut self, value: i64) -> Self {
        self.renotify_interval = Some(value);
        self
    }

    pub fn renotify_occurrences(mut self, value: i64) -> Self {
        self.renotify_occurrences = Some(value);
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

impl Default for SyntheticsMobileTestOptionsMonitorOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsMobileTestOptionsMonitorOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsMobileTestOptionsMonitorOptionsVisitor;
        impl<'a> Visitor<'a> for SyntheticsMobileTestOptionsMonitorOptionsVisitor {
            type Value = SyntheticsMobileTestOptionsMonitorOptions;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut escalation_message: Option<String> = None;
                let mut notification_preset_name: Option<crate::datadogV1::model::SyntheticsMobileTestOptionsMonitorOptionsNotificationPresetName> = None;
                let mut renotify_interval: Option<i64> = None;
                let mut renotify_occurrences: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "escalation_message" => {
                            if v.is_null() {
                                continue;
                            }
                            escalation_message =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "notification_preset_name" => {
                            if v.is_null() {
                                continue;
                            }
                            notification_preset_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _notification_preset_name) = notification_preset_name {
                                match _notification_preset_name {
                                    crate::datadogV1::model::SyntheticsMobileTestOptionsMonitorOptionsNotificationPresetName::UnparsedObject(_notification_preset_name) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "renotify_interval" => {
                            if v.is_null() {
                                continue;
                            }
                            renotify_interval =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "renotify_occurrences" => {
                            if v.is_null() {
                                continue;
                            }
                            renotify_occurrences =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsMobileTestOptionsMonitorOptions {
                    escalation_message,
                    notification_preset_name,
                    renotify_interval,
                    renotify_occurrences,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsMobileTestOptionsMonitorOptionsVisitor)
    }
}

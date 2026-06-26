// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The notification settings for a single event type on a control.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ControlNotificationEventSetting {
    /// Whether notifications are enabled for this event type.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// The event type the notification settings apply to, such as `new_detection`.
    #[serde(rename = "event_type")]
    pub event_type: String,
    /// The destinations that receive notifications for an event type.
    #[serde(rename = "targets")]
    pub targets: Vec<crate::datadogV2::model::ControlNotificationTarget>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ControlNotificationEventSetting {
    pub fn new(
        enabled: bool,
        event_type: String,
        targets: Vec<crate::datadogV2::model::ControlNotificationTarget>,
    ) -> ControlNotificationEventSetting {
        ControlNotificationEventSetting {
            enabled,
            event_type,
            targets,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for ControlNotificationEventSetting {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ControlNotificationEventSettingVisitor;
        impl<'a> Visitor<'a> for ControlNotificationEventSettingVisitor {
            type Value = ControlNotificationEventSetting;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut enabled: Option<bool> = None;
                let mut event_type: Option<String> = None;
                let mut targets: Option<Vec<crate::datadogV2::model::ControlNotificationTarget>> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "enabled" => {
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "event_type" => {
                            event_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "targets" => {
                            targets = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let enabled = enabled.ok_or_else(|| M::Error::missing_field("enabled"))?;
                let event_type = event_type.ok_or_else(|| M::Error::missing_field("event_type"))?;
                let targets = targets.ok_or_else(|| M::Error::missing_field("targets"))?;

                let content = ControlNotificationEventSetting {
                    enabled,
                    event_type,
                    targets,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ControlNotificationEventSettingVisitor)
    }
}

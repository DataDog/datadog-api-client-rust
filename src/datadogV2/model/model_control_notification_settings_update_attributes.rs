// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes of a governance control's notification settings that can be updated.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ControlNotificationSettingsUpdateAttributes {
    /// The notification settings for each supported event type on the control.
    #[serde(rename = "event_settings")]
    pub event_settings: Option<Vec<crate::datadogV2::model::ControlNotificationEventSetting>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ControlNotificationSettingsUpdateAttributes {
    pub fn new() -> ControlNotificationSettingsUpdateAttributes {
        ControlNotificationSettingsUpdateAttributes {
            event_settings: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn event_settings(
        mut self,
        value: Vec<crate::datadogV2::model::ControlNotificationEventSetting>,
    ) -> Self {
        self.event_settings = Some(value);
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

impl Default for ControlNotificationSettingsUpdateAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ControlNotificationSettingsUpdateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ControlNotificationSettingsUpdateAttributesVisitor;
        impl<'a> Visitor<'a> for ControlNotificationSettingsUpdateAttributesVisitor {
            type Value = ControlNotificationSettingsUpdateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut event_settings: Option<
                    Vec<crate::datadogV2::model::ControlNotificationEventSetting>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "event_settings" => {
                            if v.is_null() {
                                continue;
                            }
                            event_settings =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ControlNotificationSettingsUpdateAttributes {
                    event_settings,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ControlNotificationSettingsUpdateAttributesVisitor)
    }
}

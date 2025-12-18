// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for creating or modifying an on-call notification rule.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UpdateOnCallNotificationRuleRequestAttributes {
    /// Specifies the category a notification rule will apply to
    #[serde(rename = "category")]
    pub category: Option<crate::datadogV2::model::OnCallNotificationRuleCategory>,
    /// Defines the configuration for a channel associated with a notification rule
    #[serde(rename = "channel_settings")]
    pub channel_settings: Option<crate::datadogV2::model::OnCallNotificationRuleChannelSettings>,
    /// The number of minutes that will elapse before this rule is evaluated.  0 indicates immediate evaluation
    #[serde(rename = "delay_minutes")]
    pub delay_minutes: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UpdateOnCallNotificationRuleRequestAttributes {
    pub fn new() -> UpdateOnCallNotificationRuleRequestAttributes {
        UpdateOnCallNotificationRuleRequestAttributes {
            category: None,
            channel_settings: None,
            delay_minutes: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn category(
        mut self,
        value: crate::datadogV2::model::OnCallNotificationRuleCategory,
    ) -> Self {
        self.category = Some(value);
        self
    }

    pub fn channel_settings(
        mut self,
        value: crate::datadogV2::model::OnCallNotificationRuleChannelSettings,
    ) -> Self {
        self.channel_settings = Some(value);
        self
    }

    pub fn delay_minutes(mut self, value: i64) -> Self {
        self.delay_minutes = Some(value);
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

impl Default for UpdateOnCallNotificationRuleRequestAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for UpdateOnCallNotificationRuleRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UpdateOnCallNotificationRuleRequestAttributesVisitor;
        impl<'a> Visitor<'a> for UpdateOnCallNotificationRuleRequestAttributesVisitor {
            type Value = UpdateOnCallNotificationRuleRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut category: Option<crate::datadogV2::model::OnCallNotificationRuleCategory> =
                    None;
                let mut channel_settings: Option<
                    crate::datadogV2::model::OnCallNotificationRuleChannelSettings,
                > = None;
                let mut delay_minutes: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "category" => {
                            if v.is_null() {
                                continue;
                            }
                            category = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _category) = category {
                                match _category {
                                    crate::datadogV2::model::OnCallNotificationRuleCategory::UnparsedObject(_category) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "channel_settings" => {
                            if v.is_null() {
                                continue;
                            }
                            channel_settings =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _channel_settings) = channel_settings {
                                match _channel_settings {
                                    crate::datadogV2::model::OnCallNotificationRuleChannelSettings::UnparsedObject(_channel_settings) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "delay_minutes" => {
                            if v.is_null() {
                                continue;
                            }
                            delay_minutes =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = UpdateOnCallNotificationRuleRequestAttributes {
                    category,
                    channel_settings,
                    delay_minutes,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UpdateOnCallNotificationRuleRequestAttributesVisitor)
    }
}

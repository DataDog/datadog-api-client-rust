// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Configuration for using a phone notification channel in a notification rule
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OnCallPhoneNotificationRuleSettings {
    /// Specifies the method in which a phone is used in a notification rule
    #[serde(rename = "method")]
    pub method: crate::datadogV2::model::OnCallPhoneNotificationRuleMethod,
    /// Indicates that the notification channel is a phone
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::NotificationChannelPhoneConfigType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OnCallPhoneNotificationRuleSettings {
    pub fn new(
        method: crate::datadogV2::model::OnCallPhoneNotificationRuleMethod,
        type_: crate::datadogV2::model::NotificationChannelPhoneConfigType,
    ) -> OnCallPhoneNotificationRuleSettings {
        OnCallPhoneNotificationRuleSettings {
            method,
            type_,
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

impl<'de> Deserialize<'de> for OnCallPhoneNotificationRuleSettings {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OnCallPhoneNotificationRuleSettingsVisitor;
        impl<'a> Visitor<'a> for OnCallPhoneNotificationRuleSettingsVisitor {
            type Value = OnCallPhoneNotificationRuleSettings;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut method: Option<crate::datadogV2::model::OnCallPhoneNotificationRuleMethod> =
                    None;
                let mut type_: Option<crate::datadogV2::model::NotificationChannelPhoneConfigType> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "method" => {
                            method = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _method) = method {
                                match _method {
                                    crate::datadogV2::model::OnCallPhoneNotificationRuleMethod::UnparsedObject(_method) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::NotificationChannelPhoneConfigType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let method = method.ok_or_else(|| M::Error::missing_field("method"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = OnCallPhoneNotificationRuleSettings {
                    method,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OnCallPhoneNotificationRuleSettingsVisitor)
    }
}

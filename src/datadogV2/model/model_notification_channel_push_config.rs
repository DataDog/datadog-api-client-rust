// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Push notification channel configuration
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct NotificationChannelPushConfig {
    /// The name of the application used to receive push notifications
    #[serde(rename = "application_name")]
    pub application_name: String,
    /// The name of the mobile device being used
    #[serde(rename = "device_name")]
    pub device_name: String,
    /// Indicates that the notification channel is a mobile device for push notifications
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::NotificationChannelPushConfigType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl NotificationChannelPushConfig {
    pub fn new(
        application_name: String,
        device_name: String,
        type_: crate::datadogV2::model::NotificationChannelPushConfigType,
    ) -> NotificationChannelPushConfig {
        NotificationChannelPushConfig {
            application_name,
            device_name,
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

impl<'de> Deserialize<'de> for NotificationChannelPushConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NotificationChannelPushConfigVisitor;
        impl<'a> Visitor<'a> for NotificationChannelPushConfigVisitor {
            type Value = NotificationChannelPushConfig;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut application_name: Option<String> = None;
                let mut device_name: Option<String> = None;
                let mut type_: Option<crate::datadogV2::model::NotificationChannelPushConfigType> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "application_name" => {
                            application_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "device_name" => {
                            device_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::NotificationChannelPushConfigType::UnparsedObject(_type_) => {
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
                let application_name =
                    application_name.ok_or_else(|| M::Error::missing_field("application_name"))?;
                let device_name =
                    device_name.ok_or_else(|| M::Error::missing_field("device_name"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = NotificationChannelPushConfig {
                    application_name,
                    device_name,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(NotificationChannelPushConfigVisitor)
    }
}

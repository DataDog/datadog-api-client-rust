// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of the `ConfigCatSDKKey` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ConfigCatSDKKey {
    /// The `ConfigCatSDKKey` `api_password`.
    #[serde(rename = "api_password")]
    pub api_password: String,
    /// The `ConfigCatSDKKey` `api_username`.
    #[serde(rename = "api_username")]
    pub api_username: String,
    /// The `ConfigCatSDKKey` `sdk_key`.
    #[serde(rename = "sdk_key")]
    pub sdk_key: String,
    /// The definition of the `ConfigCatSDKKey` object.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ConfigCatSDKKeyType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ConfigCatSDKKey {
    pub fn new(
        api_password: String,
        api_username: String,
        sdk_key: String,
        type_: crate::datadogV2::model::ConfigCatSDKKeyType,
    ) -> ConfigCatSDKKey {
        ConfigCatSDKKey {
            api_password,
            api_username,
            sdk_key,
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

impl<'de> Deserialize<'de> for ConfigCatSDKKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ConfigCatSDKKeyVisitor;
        impl<'a> Visitor<'a> for ConfigCatSDKKeyVisitor {
            type Value = ConfigCatSDKKey;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut api_password: Option<String> = None;
                let mut api_username: Option<String> = None;
                let mut sdk_key: Option<String> = None;
                let mut type_: Option<crate::datadogV2::model::ConfigCatSDKKeyType> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "api_password" => {
                            api_password =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "api_username" => {
                            api_username =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sdk_key" => {
                            sdk_key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::ConfigCatSDKKeyType::UnparsedObject(_type_) => {
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
                let api_password =
                    api_password.ok_or_else(|| M::Error::missing_field("api_password"))?;
                let api_username =
                    api_username.ok_or_else(|| M::Error::missing_field("api_username"))?;
                let sdk_key = sdk_key.ok_or_else(|| M::Error::missing_field("sdk_key"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = ConfigCatSDKKey {
                    api_password,
                    api_username,
                    sdk_key,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ConfigCatSDKKeyVisitor)
    }
}

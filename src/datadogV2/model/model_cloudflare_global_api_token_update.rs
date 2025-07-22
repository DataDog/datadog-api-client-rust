// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of the `CloudflareGlobalAPIToken` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CloudflareGlobalAPITokenUpdate {
    /// The `CloudflareGlobalAPITokenUpdate` `auth_email`.
    #[serde(rename = "auth_email")]
    pub auth_email: Option<String>,
    /// The `CloudflareGlobalAPITokenUpdate` `global_api_key`.
    #[serde(rename = "global_api_key")]
    pub global_api_key: Option<String>,
    /// The definition of the `CloudflareGlobalAPIToken` object.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::CloudflareGlobalAPITokenType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CloudflareGlobalAPITokenUpdate {
    pub fn new(
        type_: crate::datadogV2::model::CloudflareGlobalAPITokenType,
    ) -> CloudflareGlobalAPITokenUpdate {
        CloudflareGlobalAPITokenUpdate {
            auth_email: None,
            global_api_key: None,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn auth_email(mut self, value: String) -> Self {
        self.auth_email = Some(value);
        self
    }

    pub fn global_api_key(mut self, value: String) -> Self {
        self.global_api_key = Some(value);
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

impl<'de> Deserialize<'de> for CloudflareGlobalAPITokenUpdate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CloudflareGlobalAPITokenUpdateVisitor;
        impl<'a> Visitor<'a> for CloudflareGlobalAPITokenUpdateVisitor {
            type Value = CloudflareGlobalAPITokenUpdate;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut auth_email: Option<String> = None;
                let mut global_api_key: Option<String> = None;
                let mut type_: Option<crate::datadogV2::model::CloudflareGlobalAPITokenType> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "auth_email" => {
                            if v.is_null() {
                                continue;
                            }
                            auth_email = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "global_api_key" => {
                            if v.is_null() {
                                continue;
                            }
                            global_api_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::CloudflareGlobalAPITokenType::UnparsedObject(_type_) => {
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
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = CloudflareGlobalAPITokenUpdate {
                    auth_email,
                    global_api_key,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CloudflareGlobalAPITokenUpdateVisitor)
    }
}

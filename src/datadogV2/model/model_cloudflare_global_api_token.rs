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
pub struct CloudflareGlobalAPIToken {
    /// The `CloudflareGlobalAPIToken` `auth_email`.
    #[serde(rename = "auth_email")]
    pub auth_email: String,
    /// The `CloudflareGlobalAPIToken` `global_api_key`.
    #[serde(rename = "global_api_key")]
    pub global_api_key: String,
    /// The definition of the `CloudflareGlobalAPIToken` object.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::CloudflareGlobalAPITokenType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CloudflareGlobalAPIToken {
    pub fn new(
        auth_email: String,
        global_api_key: String,
        type_: crate::datadogV2::model::CloudflareGlobalAPITokenType,
    ) -> CloudflareGlobalAPIToken {
        CloudflareGlobalAPIToken {
            auth_email,
            global_api_key,
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

impl<'de> Deserialize<'de> for CloudflareGlobalAPIToken {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CloudflareGlobalAPITokenVisitor;
        impl<'a> Visitor<'a> for CloudflareGlobalAPITokenVisitor {
            type Value = CloudflareGlobalAPIToken;

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
                            auth_email = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "global_api_key" => {
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
                let auth_email = auth_email.ok_or_else(|| M::Error::missing_field("auth_email"))?;
                let global_api_key =
                    global_api_key.ok_or_else(|| M::Error::missing_field("global_api_key"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = CloudflareGlobalAPIToken {
                    auth_email,
                    global_api_key,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CloudflareGlobalAPITokenVisitor)
    }
}

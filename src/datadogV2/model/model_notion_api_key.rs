// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of the `NotionAPIKey` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct NotionAPIKey {
    /// The `NotionAPIKey` `api_token`.
    #[serde(rename = "api_token")]
    pub api_token: String,
    /// The definition of the `NotionAPIKey` object.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::NotionAPIKeyType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl NotionAPIKey {
    pub fn new(
        api_token: String,
        type_: crate::datadogV2::model::NotionAPIKeyType,
    ) -> NotionAPIKey {
        NotionAPIKey {
            api_token,
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

impl<'de> Deserialize<'de> for NotionAPIKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NotionAPIKeyVisitor;
        impl<'a> Visitor<'a> for NotionAPIKeyVisitor {
            type Value = NotionAPIKey;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut api_token: Option<String> = None;
                let mut type_: Option<crate::datadogV2::model::NotionAPIKeyType> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "api_token" => {
                            api_token = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::NotionAPIKeyType::UnparsedObject(
                                        _type_,
                                    ) => {
                                        _unparsed = true;
                                    }
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
                let api_token = api_token.ok_or_else(|| M::Error::missing_field("api_token"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = NotionAPIKey {
                    api_token,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(NotionAPIKeyVisitor)
    }
}

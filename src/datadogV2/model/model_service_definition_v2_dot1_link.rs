// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Service's external links.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ServiceDefinitionV2Dot1Link {
    /// Link name.
    #[serde(rename = "name")]
    pub name: String,
    /// Link provider.
    #[serde(rename = "provider")]
    pub provider: Option<String>,
    /// Link type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ServiceDefinitionV2Dot1LinkType,
    /// Link URL.
    #[serde(rename = "url")]
    pub url: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ServiceDefinitionV2Dot1Link {
    pub fn new(
        name: String,
        type_: crate::datadogV2::model::ServiceDefinitionV2Dot1LinkType,
        url: String,
    ) -> ServiceDefinitionV2Dot1Link {
        ServiceDefinitionV2Dot1Link {
            name,
            provider: None,
            type_,
            url,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn provider(mut self, value: String) -> Self {
        self.provider = Some(value);
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

impl<'de> Deserialize<'de> for ServiceDefinitionV2Dot1Link {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ServiceDefinitionV2Dot1LinkVisitor;
        impl<'a> Visitor<'a> for ServiceDefinitionV2Dot1LinkVisitor {
            type Value = ServiceDefinitionV2Dot1Link;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut name: Option<String> = None;
                let mut provider: Option<String> = None;
                let mut type_: Option<crate::datadogV2::model::ServiceDefinitionV2Dot1LinkType> =
                    None;
                let mut url: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "provider" => {
                            if v.is_null() {
                                continue;
                            }
                            provider = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::ServiceDefinitionV2Dot1LinkType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "url" => {
                            url = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;
                let url = url.ok_or_else(|| M::Error::missing_field("url"))?;

                let content = ServiceDefinitionV2Dot1Link {
                    name,
                    provider,
                    type_,
                    url,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ServiceDefinitionV2Dot1LinkVisitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Google Chat organization attributes.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GoogleChatOrganizationAttributes {
    /// The Google Chat organization domain ID.
    #[serde(rename = "domain_id")]
    pub domain_id: Option<String>,
    /// The Google Chat organization domain name.
    #[serde(rename = "domain_name")]
    pub domain_name: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GoogleChatOrganizationAttributes {
    pub fn new() -> GoogleChatOrganizationAttributes {
        GoogleChatOrganizationAttributes {
            domain_id: None,
            domain_name: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn domain_id(mut self, value: String) -> Self {
        self.domain_id = Some(value);
        self
    }

    pub fn domain_name(mut self, value: String) -> Self {
        self.domain_name = Some(value);
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

impl Default for GoogleChatOrganizationAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for GoogleChatOrganizationAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GoogleChatOrganizationAttributesVisitor;
        impl<'a> Visitor<'a> for GoogleChatOrganizationAttributesVisitor {
            type Value = GoogleChatOrganizationAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut domain_id: Option<String> = None;
                let mut domain_name: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "domain_id" => {
                            if v.is_null() {
                                continue;
                            }
                            domain_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "domain_name" => {
                            if v.is_null() {
                                continue;
                            }
                            domain_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = GoogleChatOrganizationAttributes {
                    domain_id,
                    domain_name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GoogleChatOrganizationAttributesVisitor)
    }
}

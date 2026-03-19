// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Join key configuration for correlating events.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductAnalyticsJoinKeys {
    /// The primary join key facet.
    #[serde(rename = "primary")]
    pub primary: Option<String>,
    /// Secondary join key facets.
    #[serde(rename = "secondary")]
    pub secondary: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductAnalyticsJoinKeys {
    pub fn new() -> ProductAnalyticsJoinKeys {
        ProductAnalyticsJoinKeys {
            primary: None,
            secondary: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn primary(mut self, value: String) -> Self {
        self.primary = Some(value);
        self
    }

    pub fn secondary(mut self, value: Vec<String>) -> Self {
        self.secondary = Some(value);
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

impl Default for ProductAnalyticsJoinKeys {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ProductAnalyticsJoinKeys {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsJoinKeysVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsJoinKeysVisitor {
            type Value = ProductAnalyticsJoinKeys;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut primary: Option<String> = None;
                let mut secondary: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "primary" => {
                            if v.is_null() {
                                continue;
                            }
                            primary = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "secondary" => {
                            if v.is_null() {
                                continue;
                            }
                            secondary = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ProductAnalyticsJoinKeys {
                    primary,
                    secondary,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductAnalyticsJoinKeysVisitor)
    }
}

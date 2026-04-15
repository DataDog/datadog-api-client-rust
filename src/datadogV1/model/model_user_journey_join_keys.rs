// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Join keys for user journey queries.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UserJourneyJoinKeys {
    /// Primary join key.
    #[serde(rename = "primary")]
    pub primary: String,
    /// Secondary join keys.
    #[serde(rename = "secondary")]
    pub secondary: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UserJourneyJoinKeys {
    pub fn new(primary: String) -> UserJourneyJoinKeys {
        UserJourneyJoinKeys {
            primary,
            secondary: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
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

impl<'de> Deserialize<'de> for UserJourneyJoinKeys {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UserJourneyJoinKeysVisitor;
        impl<'a> Visitor<'a> for UserJourneyJoinKeysVisitor {
            type Value = UserJourneyJoinKeys;

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
                let primary = primary.ok_or_else(|| M::Error::missing_field("primary"))?;

                let content = UserJourneyJoinKeys {
                    primary,
                    secondary,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UserJourneyJoinKeysVisitor)
    }
}

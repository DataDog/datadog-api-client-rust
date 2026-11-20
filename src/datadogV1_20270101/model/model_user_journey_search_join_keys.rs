// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UserJourneySearchJoinKeys {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "primary")]
    pub primary: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "secondary")]
    pub secondary: Option<Vec<String>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl UserJourneySearchJoinKeys {
    pub fn new(primary: String) -> UserJourneySearchJoinKeys {
        UserJourneySearchJoinKeys {
            primary,
            secondary: None,
            _unparsed: false,
        }
    }
    
    pub fn secondary(mut self, value: Vec<String>) -> Self {
        self.secondary = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for UserJourneySearchJoinKeys {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UserJourneySearchJoinKeysVisitor;
        impl<'a> Visitor<'a> for UserJourneySearchJoinKeysVisitor {
            type Value = UserJourneySearchJoinKeys;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut primary: Option<String> = None;
                let mut secondary: Option<Vec<String>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "primary" => {
                            primary = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "secondary" => {
                            if v.is_null() {
                                continue;
                            }
                            secondary = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let primary = primary.ok_or_else(|| M::Error::missing_field("primary"))?;

                let content = UserJourneySearchJoinKeys {
                    primary,
                    secondary,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UserJourneySearchJoinKeysVisitor)
    }
}

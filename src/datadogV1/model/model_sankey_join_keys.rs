// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Join keys.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SankeyJoinKeys {
    /// Primary join key.
    #[serde(rename = "primary")]
    pub primary: String,
    /// Secondary join keys.
    #[serde(rename = "secondary")]
    pub secondary: Option<Vec<String>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SankeyJoinKeys {
    pub fn new(primary: String) -> SankeyJoinKeys {
        SankeyJoinKeys {
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

impl<'de> Deserialize<'de> for SankeyJoinKeys {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SankeyJoinKeysVisitor;
        impl<'a> Visitor<'a> for SankeyJoinKeysVisitor {
            type Value = SankeyJoinKeys;

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
                        }
                        "secondary" => {
                            if v.is_null() {
                                continue;
                            }
                            secondary = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let primary = primary.ok_or_else(|| M::Error::missing_field("primary"))?;

                let content = SankeyJoinKeys {
                    primary,
                    secondary,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SankeyJoinKeysVisitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// An EventBridge source.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AWSEventBridgeSource {
    /// The event source name.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The event source's [AWS region](<https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints>).
    #[serde(rename = "region")]
    pub region: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AWSEventBridgeSource {
    pub fn new() -> AWSEventBridgeSource {
        AWSEventBridgeSource {
            name: None,
            region: None,
            _unparsed: false,
        }
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn region(mut self, value: String) -> Self {
        self.region = Some(value);
        self
    }
}

impl Default for AWSEventBridgeSource {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AWSEventBridgeSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AWSEventBridgeSourceVisitor;
        impl<'a> Visitor<'a> for AWSEventBridgeSourceVisitor {
            type Value = AWSEventBridgeSource;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut name: Option<String> = None;
                let mut region: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "region" => {
                            if v.is_null() {
                                continue;
                            }
                            region = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = AWSEventBridgeSource {
                    name,
                    region,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AWSEventBridgeSourceVisitor)
    }
}

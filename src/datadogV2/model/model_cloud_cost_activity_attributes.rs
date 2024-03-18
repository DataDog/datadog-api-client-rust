// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for Cloud Cost activity.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CloudCostActivityAttributes {
    /// Whether or not the cloud account is enabled.
    #[serde(rename = "is_enabled")]
    pub is_enabled: bool,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CloudCostActivityAttributes {
    pub fn new(is_enabled: bool) -> CloudCostActivityAttributes {
        CloudCostActivityAttributes {
            is_enabled,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for CloudCostActivityAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CloudCostActivityAttributesVisitor;
        impl<'a> Visitor<'a> for CloudCostActivityAttributesVisitor {
            type Value = CloudCostActivityAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut is_enabled: Option<bool> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "is_enabled" => {
                            is_enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let is_enabled = is_enabled.ok_or_else(|| M::Error::missing_field("is_enabled"))?;

                let content = CloudCostActivityAttributes {
                    is_enabled,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CloudCostActivityAttributesVisitor)
    }
}

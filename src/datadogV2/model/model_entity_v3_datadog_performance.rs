// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Performance stats association.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct EntityV3DatadogPerformance {
    /// A list of APM entity tags that associates the APM Stats data with the entity.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl EntityV3DatadogPerformance {
    pub fn new() -> EntityV3DatadogPerformance {
        EntityV3DatadogPerformance {
            tags: None,
            _unparsed: false,
        }
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }
}

impl Default for EntityV3DatadogPerformance {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for EntityV3DatadogPerformance {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EntityV3DatadogPerformanceVisitor;
        impl<'a> Visitor<'a> for EntityV3DatadogPerformanceVisitor {
            type Value = EntityV3DatadogPerformance;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut tags: Option<Vec<String>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }

                let content = EntityV3DatadogPerformance { tags, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(EntityV3DatadogPerformanceVisitor)
    }
}

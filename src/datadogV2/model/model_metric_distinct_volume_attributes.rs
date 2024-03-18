// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object containing the definition of a metric's distinct volume.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MetricDistinctVolumeAttributes {
    /// Distinct volume for the given metric.
    #[serde(rename = "distinct_volume")]
    pub distinct_volume: Option<i64>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MetricDistinctVolumeAttributes {
    pub fn new() -> MetricDistinctVolumeAttributes {
        MetricDistinctVolumeAttributes {
            distinct_volume: None,
            _unparsed: false,
        }
    }

    pub fn distinct_volume(mut self, value: i64) -> Self {
        self.distinct_volume = Some(value);
        self
    }
}

impl Default for MetricDistinctVolumeAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for MetricDistinctVolumeAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MetricDistinctVolumeAttributesVisitor;
        impl<'a> Visitor<'a> for MetricDistinctVolumeAttributesVisitor {
            type Value = MetricDistinctVolumeAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut distinct_volume: Option<i64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "distinct_volume" => {
                            if v.is_null() {
                                continue;
                            }
                            distinct_volume =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = MetricDistinctVolumeAttributes {
                    distinct_volume,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MetricDistinctVolumeAttributesVisitor)
    }
}

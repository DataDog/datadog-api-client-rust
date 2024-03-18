// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object containing the definition of a metric's ingested and indexed volume.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MetricIngestedIndexedVolumeAttributes {
    /// Indexed volume for the given metric.
    #[serde(rename = "indexed_volume")]
    pub indexed_volume: Option<i64>,
    /// Ingested volume for the given metric.
    #[serde(rename = "ingested_volume")]
    pub ingested_volume: Option<i64>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MetricIngestedIndexedVolumeAttributes {
    pub fn new() -> MetricIngestedIndexedVolumeAttributes {
        MetricIngestedIndexedVolumeAttributes {
            indexed_volume: None,
            ingested_volume: None,
            _unparsed: false,
        }
    }

    pub fn indexed_volume(mut self, value: i64) -> Self {
        self.indexed_volume = Some(value);
        self
    }

    pub fn ingested_volume(mut self, value: i64) -> Self {
        self.ingested_volume = Some(value);
        self
    }
}

impl Default for MetricIngestedIndexedVolumeAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for MetricIngestedIndexedVolumeAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MetricIngestedIndexedVolumeAttributesVisitor;
        impl<'a> Visitor<'a> for MetricIngestedIndexedVolumeAttributesVisitor {
            type Value = MetricIngestedIndexedVolumeAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut indexed_volume: Option<i64> = None;
                let mut ingested_volume: Option<i64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "indexed_volume" => {
                            if v.is_null() {
                                continue;
                            }
                            indexed_volume =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ingested_volume" => {
                            if v.is_null() {
                                continue;
                            }
                            ingested_volume =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = MetricIngestedIndexedVolumeAttributes {
                    indexed_volume,
                    ingested_volume,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MetricIngestedIndexedVolumeAttributesVisitor)
    }
}

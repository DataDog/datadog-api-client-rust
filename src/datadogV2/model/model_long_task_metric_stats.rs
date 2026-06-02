// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Statistical distribution (average, min, max) of a long task metric across sampled views.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LongTaskMetricStats {
    /// Average value across sampled views.
    #[serde(rename = "average")]
    pub average: f64,
    /// Maximum value across sampled views.
    #[serde(rename = "max")]
    pub max: f64,
    /// Minimum value across sampled views.
    #[serde(rename = "min")]
    pub min: f64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LongTaskMetricStats {
    pub fn new(average: f64, max: f64, min: f64) -> LongTaskMetricStats {
        LongTaskMetricStats {
            average,
            max,
            min,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for LongTaskMetricStats {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LongTaskMetricStatsVisitor;
        impl<'a> Visitor<'a> for LongTaskMetricStatsVisitor {
            type Value = LongTaskMetricStats;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut average: Option<f64> = None;
                let mut max: Option<f64> = None;
                let mut min: Option<f64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "average" => {
                            average = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "max" => {
                            max = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "min" => {
                            min = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let average = average.ok_or_else(|| M::Error::missing_field("average"))?;
                let max = max.ok_or_else(|| M::Error::missing_field("max"))?;
                let min = min.ok_or_else(|| M::Error::missing_field("min"))?;

                let content = LongTaskMetricStats {
                    average,
                    max,
                    min,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LongTaskMetricStatsVisitor)
    }
}

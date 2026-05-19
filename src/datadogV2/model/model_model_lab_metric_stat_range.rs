// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The range of values for a specific metric statistic.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ModelLabMetricStatRange {
    /// The maximum value of the statistic.
    #[serde(rename = "max")]
    pub max: f64,
    /// The minimum value of the statistic.
    #[serde(rename = "min")]
    pub min: f64,
    /// The metric statistic name.
    #[serde(rename = "stat")]
    pub stat: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ModelLabMetricStatRange {
    pub fn new(max: f64, min: f64, stat: String) -> ModelLabMetricStatRange {
        ModelLabMetricStatRange {
            max,
            min,
            stat,
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

impl<'de> Deserialize<'de> for ModelLabMetricStatRange {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ModelLabMetricStatRangeVisitor;
        impl<'a> Visitor<'a> for ModelLabMetricStatRangeVisitor {
            type Value = ModelLabMetricStatRange;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut max: Option<f64> = None;
                let mut min: Option<f64> = None;
                let mut stat: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "max" => {
                            max = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "min" => {
                            min = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "stat" => {
                            stat = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let max = max.ok_or_else(|| M::Error::missing_field("max"))?;
                let min = min.ok_or_else(|| M::Error::missing_field("min"))?;
                let stat = stat.ok_or_else(|| M::Error::missing_field("stat"))?;

                let content = ModelLabMetricStatRange {
                    max,
                    min,
                    stat,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ModelLabMetricStatRangeVisitor)
    }
}

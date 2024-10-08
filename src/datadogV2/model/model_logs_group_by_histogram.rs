// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Used to perform a histogram computation (only for measure facets).
/// Note: at most 100 buckets are allowed, the number of buckets is (max - min)/interval.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LogsGroupByHistogram {
    /// The bin size of the histogram buckets
    #[serde(rename = "interval")]
    pub interval: f64,
    /// The maximum value for the measure used in the histogram
    /// (values greater than this one are filtered out)
    #[serde(rename = "max")]
    pub max: f64,
    /// The minimum value for the measure used in the histogram
    /// (values smaller than this one are filtered out)
    #[serde(rename = "min")]
    pub min: f64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LogsGroupByHistogram {
    pub fn new(interval: f64, max: f64, min: f64) -> LogsGroupByHistogram {
        LogsGroupByHistogram {
            interval,
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

impl<'de> Deserialize<'de> for LogsGroupByHistogram {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogsGroupByHistogramVisitor;
        impl<'a> Visitor<'a> for LogsGroupByHistogramVisitor {
            type Value = LogsGroupByHistogram;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut interval: Option<f64> = None;
                let mut max: Option<f64> = None;
                let mut min: Option<f64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "interval" => {
                            interval = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let interval = interval.ok_or_else(|| M::Error::missing_field("interval"))?;
                let max = max.ok_or_else(|| M::Error::missing_field("max"))?;
                let min = min.ok_or_else(|| M::Error::missing_field("min"))?;

                let content = LogsGroupByHistogram {
                    interval,
                    max,
                    min,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogsGroupByHistogramVisitor)
    }
}

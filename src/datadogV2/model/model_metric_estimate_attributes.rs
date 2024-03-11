// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object containing the definition of a metric estimate attribute.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MetricEstimateAttributes {
    /// Estimate type based on the queried configuration. By default, `count_or_gauge` is returned. `distribution` is returned for distribution metrics without percentiles enabled. Lastly, `percentile` is returned if `filter[pct]=true` is queried with a distribution metric.
    #[serde(rename = "estimate_type")]
    pub estimate_type: Option<crate::datadogV2::model::MetricEstimateType>,
    /// Timestamp when the cardinality estimate was requested.
    #[serde(rename = "estimated_at")]
    pub estimated_at: Option<String>,
    /// Estimated cardinality of the metric based on the queried configuration.
    #[serde(rename = "estimated_output_series")]
    pub estimated_output_series: Option<i64>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MetricEstimateAttributes {
    pub fn new() -> MetricEstimateAttributes {
        MetricEstimateAttributes {
            estimate_type: None,
            estimated_at: None,
            estimated_output_series: None,
            _unparsed: false,
        }
    }

    pub fn estimate_type(
        &mut self,
        value: crate::datadogV2::model::MetricEstimateType,
    ) -> &mut Self {
        self.estimate_type = Some(value);
        self
    }

    pub fn estimated_at(&mut self, value: String) -> &mut Self {
        self.estimated_at = Some(value);
        self
    }

    pub fn estimated_output_series(&mut self, value: i64) -> &mut Self {
        self.estimated_output_series = Some(value);
        self
    }
}

impl Default for MetricEstimateAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for MetricEstimateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MetricEstimateAttributesVisitor;
        impl<'a> Visitor<'a> for MetricEstimateAttributesVisitor {
            type Value = MetricEstimateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut estimate_type: Option<crate::datadogV2::model::MetricEstimateType> = None;
                let mut estimated_at: Option<String> = None;
                let mut estimated_output_series: Option<i64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "estimate_type" => {
                            if v.is_null() {
                                continue;
                            }
                            estimate_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _estimate_type) = estimate_type {
                                match _estimate_type {
                                    crate::datadogV2::model::MetricEstimateType::UnparsedObject(
                                        _estimate_type,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "estimated_at" => {
                            if v.is_null() {
                                continue;
                            }
                            estimated_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "estimated_output_series" => {
                            if v.is_null() {
                                continue;
                            }
                            estimated_output_series =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = MetricEstimateAttributes {
                    estimate_type,
                    estimated_at,
                    estimated_output_series,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MetricEstimateAttributesVisitor)
    }
}

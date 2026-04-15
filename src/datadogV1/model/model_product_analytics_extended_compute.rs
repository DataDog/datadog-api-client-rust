// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Compute configuration for Product Analytics Extended queries.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductAnalyticsExtendedCompute {
    /// Aggregation methods for event platform queries.
    #[serde(rename = "aggregation")]
    pub aggregation: crate::datadogV1::model::FormulaAndFunctionEventAggregation,
    /// Fixed-width time bucket interval in milliseconds for time series queries. Mutually exclusive with `rollup`.
    #[serde(rename = "interval")]
    pub interval: Option<f64>,
    /// Measurable attribute to compute.
    #[serde(rename = "metric")]
    pub metric: Option<String>,
    /// Name of the compute for use in formulas.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Calendar interval definition.
    #[serde(rename = "rollup")]
    pub rollup: Option<crate::datadogV1::model::CalendarInterval>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductAnalyticsExtendedCompute {
    pub fn new(
        aggregation: crate::datadogV1::model::FormulaAndFunctionEventAggregation,
    ) -> ProductAnalyticsExtendedCompute {
        ProductAnalyticsExtendedCompute {
            aggregation,
            interval: None,
            metric: None,
            name: None,
            rollup: None,
            _unparsed: false,
        }
    }

    pub fn interval(mut self, value: f64) -> Self {
        self.interval = Some(value);
        self
    }

    pub fn metric(mut self, value: String) -> Self {
        self.metric = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn rollup(mut self, value: crate::datadogV1::model::CalendarInterval) -> Self {
        self.rollup = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for ProductAnalyticsExtendedCompute {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsExtendedComputeVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsExtendedComputeVisitor {
            type Value = ProductAnalyticsExtendedCompute;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut aggregation: Option<
                    crate::datadogV1::model::FormulaAndFunctionEventAggregation,
                > = None;
                let mut interval: Option<f64> = None;
                let mut metric: Option<String> = None;
                let mut name: Option<String> = None;
                let mut rollup: Option<crate::datadogV1::model::CalendarInterval> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "aggregation" => {
                            aggregation =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _aggregation) = aggregation {
                                match _aggregation {
                                    crate::datadogV1::model::FormulaAndFunctionEventAggregation::UnparsedObject(_aggregation) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "interval" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            interval = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metric" => {
                            if v.is_null() {
                                continue;
                            }
                            metric = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rollup" => {
                            if v.is_null() {
                                continue;
                            }
                            rollup = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let aggregation =
                    aggregation.ok_or_else(|| M::Error::missing_field("aggregation"))?;

                let content = ProductAnalyticsExtendedCompute {
                    aggregation,
                    interval,
                    metric,
                    name,
                    rollup,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductAnalyticsExtendedComputeVisitor)
    }
}

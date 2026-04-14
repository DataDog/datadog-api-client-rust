// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Compute configuration for User Journey formula queries.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UserJourneyFormulaCompute {
    /// Aggregation methods for event platform queries.
    #[serde(rename = "aggregation")]
    pub aggregation: crate::datadogV1::model::FormulaAndFunctionEventAggregation,
    /// Time bucket interval in milliseconds for time series queries.
    #[serde(rename = "interval")]
    pub interval: Option<f64>,
    /// Metric for User Journey formula compute. `__dd.conversion` and `__dd.conversion_rate` accept `count` and `cardinality` as aggregations. `__dd.time_to_convert` accepts `avg`, `median`, `pc75`, `pc95`, `pc98`, `pc99`, `min`, and `max`.
    #[serde(rename = "metric")]
    pub metric: Option<crate::datadogV1::model::UserJourneyFormulaComputeMetric>,
    /// Target for user journey search.
    #[serde(rename = "target")]
    pub target: Option<crate::datadogV1::model::UserJourneySearchTarget>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UserJourneyFormulaCompute {
    pub fn new(
        aggregation: crate::datadogV1::model::FormulaAndFunctionEventAggregation,
    ) -> UserJourneyFormulaCompute {
        UserJourneyFormulaCompute {
            aggregation,
            interval: None,
            metric: None,
            target: None,
            _unparsed: false,
        }
    }

    pub fn interval(mut self, value: f64) -> Self {
        self.interval = Some(value);
        self
    }

    pub fn metric(
        mut self,
        value: crate::datadogV1::model::UserJourneyFormulaComputeMetric,
    ) -> Self {
        self.metric = Some(value);
        self
    }

    pub fn target(mut self, value: crate::datadogV1::model::UserJourneySearchTarget) -> Self {
        self.target = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for UserJourneyFormulaCompute {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UserJourneyFormulaComputeVisitor;
        impl<'a> Visitor<'a> for UserJourneyFormulaComputeVisitor {
            type Value = UserJourneyFormulaCompute;

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
                let mut metric: Option<crate::datadogV1::model::UserJourneyFormulaComputeMetric> =
                    None;
                let mut target: Option<crate::datadogV1::model::UserJourneySearchTarget> = None;
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
                            if let Some(ref _metric) = metric {
                                match _metric {
                                    crate::datadogV1::model::UserJourneyFormulaComputeMetric::UnparsedObject(_metric) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "target" => {
                            if v.is_null() {
                                continue;
                            }
                            target = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = UserJourneyFormulaCompute {
                    aggregation,
                    interval,
                    metric,
                    target,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UserJourneyFormulaComputeVisitor)
    }
}

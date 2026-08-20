// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Defines the metric computed over the journey.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductAnalyticsGraphQueryCompute {
    /// Aggregation function: `count`, `cardinality`, `avg`, `median`, `min`, `max`, `sum`,
    /// or a percentile of the form `pc<N>` such as `pc95`. Defaults to `cardinality`.
    #[serde(rename = "aggregation")]
    pub aggregation: String,
    /// Time bucket interval in milliseconds, used by timeseries queries.
    #[serde(rename = "interval")]
    pub interval: Option<i64>,
    /// Metric to aggregate on. Use a facet path such as `@view.time_spent`, or one of the
    /// journey metrics `__dd.conversion`, `__dd.conversion_rate`, `__dd.time_to_convert`,
    /// or `__dd.dropoff_rate`. Defaults to `__dd.conversion`.
    #[serde(rename = "metric")]
    pub metric: Option<String>,
    /// A reference to a step, or a range of steps, in the journey.
    /// Use a `node` target to name a single step, or a `path` target to name the range
    /// between two steps.
    #[serde(rename = "target")]
    pub target: Option<crate::datadogV2::model::ProductAnalyticsJourneyTarget>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductAnalyticsGraphQueryCompute {
    pub fn new(aggregation: String) -> ProductAnalyticsGraphQueryCompute {
        ProductAnalyticsGraphQueryCompute {
            aggregation,
            interval: None,
            metric: None,
            target: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn interval(mut self, value: i64) -> Self {
        self.interval = Some(value);
        self
    }

    pub fn metric(mut self, value: String) -> Self {
        self.metric = Some(value);
        self
    }

    pub fn target(mut self, value: crate::datadogV2::model::ProductAnalyticsJourneyTarget) -> Self {
        self.target = Some(value);
        self
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for ProductAnalyticsGraphQueryCompute {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsGraphQueryComputeVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsGraphQueryComputeVisitor {
            type Value = ProductAnalyticsGraphQueryCompute;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut aggregation: Option<String> = None;
                let mut interval: Option<i64> = None;
                let mut metric: Option<String> = None;
                let mut target: Option<crate::datadogV2::model::ProductAnalyticsJourneyTarget> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "aggregation" => {
                            aggregation =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "interval" => {
                            if v.is_null() {
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
                        "target" => {
                            if v.is_null() {
                                continue;
                            }
                            target = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _target) = target {
                                match _target {
                                    crate::datadogV2::model::ProductAnalyticsJourneyTarget::UnparsedObject(_target) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let aggregation =
                    aggregation.ok_or_else(|| M::Error::missing_field("aggregation"))?;

                let content = ProductAnalyticsGraphQueryCompute {
                    aggregation,
                    interval,
                    metric,
                    target,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductAnalyticsGraphQueryComputeVisitor)
    }
}

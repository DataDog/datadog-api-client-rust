// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductAnalyticsCompute {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "aggregation")]
    pub aggregation: crate::datadogV1_20270101::model::EventsAggregator,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "interval")]
    pub interval: Option<f64>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "metric")]
    pub metric: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "rollup")]
    pub rollup: Option<crate::datadogV1_20270101::model::CalendarInterval>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "source")]
    pub source: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ProductAnalyticsCompute {
    pub fn new(aggregation: crate::datadogV1_20270101::model::EventsAggregator) -> ProductAnalyticsCompute {
        ProductAnalyticsCompute {
            aggregation,
            interval: None,
            metric: None,
            rollup: None,
            source: None,
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
    
    pub fn rollup(mut self, value: crate::datadogV1_20270101::model::CalendarInterval) -> Self {
        self.rollup = Some(value);
        self
    }
    
    pub fn source(mut self, value: String) -> Self {
        self.source = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for ProductAnalyticsCompute {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsComputeVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsComputeVisitor {
            type Value = ProductAnalyticsCompute;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut aggregation: Option<crate::datadogV1_20270101::model::EventsAggregator> = None;
                let mut interval: Option<f64> = None;
                let mut metric: Option<String> = None;
                let mut rollup: Option<crate::datadogV1_20270101::model::CalendarInterval> = None;
                let mut source: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "aggregation" => {
                            aggregation = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _aggregation) = aggregation {
                                match _aggregation {
                                    crate::datadogV1_20270101::model::EventsAggregator::UnparsedObject(_aggregation) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "interval" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            interval = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "metric" => {
                            if v.is_null() {
                                continue;
                            }
                            metric = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "rollup" => {
                            if v.is_null() {
                                continue;
                            }
                            rollup = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _rollup) = rollup {
                                match _rollup {
                                    crate::datadogV1_20270101::model::CalendarInterval::UnparsedObject(_rollup) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "source" => {
                            if v.is_null() {
                                continue;
                            }
                            source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let aggregation = aggregation.ok_or_else(|| M::Error::missing_field("aggregation"))?;

                let content = ProductAnalyticsCompute {
                    aggregation,
                    interval,
                    metric,
                    rollup,
                    source,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductAnalyticsComputeVisitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Compute configuration for retention queries.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RetentionCompute {
    /// The type of aggregation that can be performed on events-based queries.
    #[serde(rename = "aggregation")]
    pub aggregation: crate::datadogV1::model::EventsAggregation,
    /// Metric for retention compute.
    #[serde(rename = "metric")]
    pub metric: crate::datadogV1::model::RetentionComputeMetric,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RetentionCompute {
    pub fn new(
        aggregation: crate::datadogV1::model::EventsAggregation,
        metric: crate::datadogV1::model::RetentionComputeMetric,
    ) -> RetentionCompute {
        RetentionCompute {
            aggregation,
            metric,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for RetentionCompute {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RetentionComputeVisitor;
        impl<'a> Visitor<'a> for RetentionComputeVisitor {
            type Value = RetentionCompute;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut aggregation: Option<crate::datadogV1::model::EventsAggregation> = None;
                let mut metric: Option<crate::datadogV1::model::RetentionComputeMetric> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "aggregation" => {
                            aggregation =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _aggregation) = aggregation {
                                match _aggregation {
                                    crate::datadogV1::model::EventsAggregation::UnparsedObject(
                                        _aggregation,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "metric" => {
                            metric = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _metric) = metric {
                                match _metric {
                                    crate::datadogV1::model::RetentionComputeMetric::UnparsedObject(_metric) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
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
                let metric = metric.ok_or_else(|| M::Error::missing_field("metric"))?;

                let content = RetentionCompute {
                    aggregation,
                    metric,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RetentionComputeVisitor)
    }
}

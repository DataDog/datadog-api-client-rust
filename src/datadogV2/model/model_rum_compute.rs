// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A compute rule to compute metrics or timeseries.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RUMCompute {
    /// An aggregation function.
    #[serde(rename = "aggregation")]
    pub aggregation: crate::datadogV2::model::RUMAggregationFunction,
    /// The time buckets' size (only used for type=timeseries)
    /// Defaults to a resolution of 150 points.
    #[serde(rename = "interval")]
    pub interval: Option<String>,
    /// The metric to use.
    #[serde(rename = "metric")]
    pub metric: Option<String>,
    /// The type of compute.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::RUMComputeType>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RUMCompute {
    pub fn new(aggregation: crate::datadogV2::model::RUMAggregationFunction) -> RUMCompute {
        RUMCompute {
            aggregation,
            interval: None,
            metric: None,
            type_: None,
            _unparsed: false,
        }
    }

    pub fn interval(mut self, value: String) -> Self {
        self.interval = Some(value);
        self
    }

    pub fn metric(mut self, value: String) -> Self {
        self.metric = Some(value);
        self
    }

    pub fn type_(mut self, value: crate::datadogV2::model::RUMComputeType) -> Self {
        self.type_ = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for RUMCompute {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RUMComputeVisitor;
        impl<'a> Visitor<'a> for RUMComputeVisitor {
            type Value = RUMCompute;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut aggregation: Option<crate::datadogV2::model::RUMAggregationFunction> = None;
                let mut interval: Option<String> = None;
                let mut metric: Option<String> = None;
                let mut type_: Option<crate::datadogV2::model::RUMComputeType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "aggregation" => {
                            aggregation =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _aggregation) = aggregation {
                                match _aggregation {
                                    crate::datadogV2::model::RUMAggregationFunction::UnparsedObject(_aggregation) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
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
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::RUMComputeType::UnparsedObject(
                                        _type_,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }
                let aggregation =
                    aggregation.ok_or_else(|| M::Error::missing_field("aggregation"))?;

                let content = RUMCompute {
                    aggregation,
                    interval,
                    metric,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RUMComputeVisitor)
    }
}

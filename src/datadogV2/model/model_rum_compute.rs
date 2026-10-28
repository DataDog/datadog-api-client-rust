// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A compute rule to compute metrics or timeseries. In the response, the result of each rule is keyed `c0`, `c1`... following the order of the `compute` list.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RUMCompute {
    /// An aggregation function. `count` counts events and needs no metric. `cardinality` counts the distinct values of the metric. `sum`, `min`, `max`, `avg`, `median` and the percentiles (`pc75` to `pc99`) apply to a numeric metric such as `@view.loading_time`.
    #[serde(rename = "aggregation")]
    pub aggregation: crate::datadogV2::model::RUMAggregationFunction,
    /// The time buckets' size, for example `5m`, `1h` or `1d` (only used for `type=timeseries`).
    /// Defaults to a resolution of 150 points over the queried time range.
    #[serde(rename = "interval")]
    pub interval: Option<String>,
    /// The attribute to aggregate, for example `@view.loading_time`. Not needed for `count`, required for the other aggregations.
    #[serde(rename = "metric")]
    pub metric: Option<String>,
    /// The type of compute. `total` returns one value per bucket, `timeseries` returns a list of time points per bucket.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::RUMComputeType>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
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
            additional_properties: std::collections::BTreeMap::new(),
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

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
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
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let aggregation =
                    aggregation.ok_or_else(|| M::Error::missing_field("aggregation"))?;

                let content = RUMCompute {
                    aggregation,
                    interval,
                    metric,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RUMComputeVisitor)
    }
}

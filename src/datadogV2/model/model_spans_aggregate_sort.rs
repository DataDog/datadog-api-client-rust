// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A sort rule.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SpansAggregateSort {
    /// An aggregation function.
    #[serde(rename = "aggregation")]
    pub aggregation: Option<crate::datadogV2::model::SpansAggregationFunction>,
    /// The metric to sort by (only used for `type=measure`).
    #[serde(rename = "metric")]
    pub metric: Option<String>,
    /// The order to use, ascending or descending.
    #[serde(rename = "order")]
    pub order: Option<crate::datadogV2::model::SpansSortOrder>,
    /// The type of sorting algorithm.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::SpansAggregateSortType>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SpansAggregateSort {
    pub fn new() -> SpansAggregateSort {
        SpansAggregateSort {
            aggregation: None,
            metric: None,
            order: None,
            type_: None,
            _unparsed: false,
        }
    }

    pub fn aggregation(mut self, value: crate::datadogV2::model::SpansAggregationFunction) -> Self {
        self.aggregation = Some(value);
        self
    }

    pub fn metric(mut self, value: String) -> Self {
        self.metric = Some(value);
        self
    }

    pub fn order(mut self, value: crate::datadogV2::model::SpansSortOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn type_(mut self, value: crate::datadogV2::model::SpansAggregateSortType) -> Self {
        self.type_ = Some(value);
        self
    }
}

impl Default for SpansAggregateSort {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SpansAggregateSort {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SpansAggregateSortVisitor;
        impl<'a> Visitor<'a> for SpansAggregateSortVisitor {
            type Value = SpansAggregateSort;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut aggregation: Option<crate::datadogV2::model::SpansAggregationFunction> =
                    None;
                let mut metric: Option<String> = None;
                let mut order: Option<crate::datadogV2::model::SpansSortOrder> = None;
                let mut type_: Option<crate::datadogV2::model::SpansAggregateSortType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "aggregation" => {
                            if v.is_null() {
                                continue;
                            }
                            aggregation =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _aggregation) = aggregation {
                                match _aggregation {
                                    crate::datadogV2::model::SpansAggregationFunction::UnparsedObject(_aggregation) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "metric" => {
                            if v.is_null() {
                                continue;
                            }
                            metric = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "order" => {
                            if v.is_null() {
                                continue;
                            }
                            order = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _order) = order {
                                match _order {
                                    crate::datadogV2::model::SpansSortOrder::UnparsedObject(
                                        _order,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::SpansAggregateSortType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }

                let content = SpansAggregateSort {
                    aggregation,
                    metric,
                    order,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SpansAggregateSortVisitor)
    }
}

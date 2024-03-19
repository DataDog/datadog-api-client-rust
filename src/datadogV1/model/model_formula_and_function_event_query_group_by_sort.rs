// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Options for sorting group by results.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FormulaAndFunctionEventQueryGroupBySort {
    /// Aggregation methods for event platform queries.
    #[serde(rename = "aggregation")]
    pub aggregation: crate::datadogV1::model::FormulaAndFunctionEventAggregation,
    /// Metric used for sorting group by results.
    #[serde(rename = "metric")]
    pub metric: Option<String>,
    /// Direction of sort.
    #[serde(rename = "order")]
    pub order: Option<crate::datadogV1::model::QuerySortOrder>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FormulaAndFunctionEventQueryGroupBySort {
    pub fn new(
        aggregation: crate::datadogV1::model::FormulaAndFunctionEventAggregation,
    ) -> FormulaAndFunctionEventQueryGroupBySort {
        FormulaAndFunctionEventQueryGroupBySort {
            aggregation,
            metric: None,
            order: None,
            _unparsed: false,
        }
    }

    pub fn metric(mut self, value: String) -> Self {
        self.metric = Some(value);
        self
    }

    pub fn order(mut self, value: crate::datadogV1::model::QuerySortOrder) -> Self {
        self.order = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for FormulaAndFunctionEventQueryGroupBySort {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FormulaAndFunctionEventQueryGroupBySortVisitor;
        impl<'a> Visitor<'a> for FormulaAndFunctionEventQueryGroupBySortVisitor {
            type Value = FormulaAndFunctionEventQueryGroupBySort;

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
                let mut metric: Option<String> = None;
                let mut order: Option<crate::datadogV1::model::QuerySortOrder> = None;
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
                                    crate::datadogV1::model::QuerySortOrder::UnparsedObject(
                                        _order,
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

                let content = FormulaAndFunctionEventQueryGroupBySort {
                    aggregation,
                    metric,
                    order,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FormulaAndFunctionEventQueryGroupBySortVisitor)
    }
}

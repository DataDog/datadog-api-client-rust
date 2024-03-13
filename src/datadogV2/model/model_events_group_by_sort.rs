// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The dimension by which to sort a query's results.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct EventsGroupBySort {
    /// The type of aggregation that can be performed on events-based queries.
    #[serde(rename = "aggregation")]
    pub aggregation: crate::datadogV2::model::EventsAggregation,
    /// The metric's calculated value which should be used to define the sort order of a query's results.
    #[serde(rename = "metric")]
    pub metric: Option<String>,
    /// Direction of sort.
    #[serde(rename = "order")]
    pub order: Option<crate::datadogV2::model::QuerySortOrder>,
    /// The type of sort to use on the calculated value.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::EventsSortType>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl EventsGroupBySort {
    pub fn new(aggregation: crate::datadogV2::model::EventsAggregation) -> EventsGroupBySort {
        EventsGroupBySort {
            aggregation,
            metric: None,
            order: None,
            type_: None,
            _unparsed: false,
        }
    }

    pub fn metric(mut self, value: String) -> Self {
        self.metric = Some(value);
        self
    }

    pub fn order(mut self, value: crate::datadogV2::model::QuerySortOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn type_(mut self, value: crate::datadogV2::model::EventsSortType) -> Self {
        self.type_ = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for EventsGroupBySort {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EventsGroupBySortVisitor;
        impl<'a> Visitor<'a> for EventsGroupBySortVisitor {
            type Value = EventsGroupBySort;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut aggregation: Option<crate::datadogV2::model::EventsAggregation> = None;
                let mut metric: Option<String> = None;
                let mut order: Option<crate::datadogV2::model::QuerySortOrder> = None;
                let mut type_: Option<crate::datadogV2::model::EventsSortType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "aggregation" => {
                            aggregation =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _aggregation) = aggregation {
                                match _aggregation {
                                    crate::datadogV2::model::EventsAggregation::UnparsedObject(
                                        _aggregation,
                                    ) => {
                                        _unparsed = true;
                                    }
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
                                    crate::datadogV2::model::QuerySortOrder::UnparsedObject(
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
                                    crate::datadogV2::model::EventsSortType::UnparsedObject(
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

                let content = EventsGroupBySort {
                    aggregation,
                    metric,
                    order,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(EventsGroupBySortVisitor)
    }
}

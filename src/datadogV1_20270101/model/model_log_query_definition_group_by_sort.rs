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
pub struct LogQueryDefinitionGroupBySort {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "aggregation")]
    pub aggregation: crate::datadogV1_20270101::model::EventsAggregator,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "facet")]
    pub facet: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "order")]
    pub order: crate::datadogV1_20270101::model::QuerySortOrder,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl LogQueryDefinitionGroupBySort {
    pub fn new(aggregation: crate::datadogV1_20270101::model::EventsAggregator, order: crate::datadogV1_20270101::model::QuerySortOrder) -> LogQueryDefinitionGroupBySort {
        LogQueryDefinitionGroupBySort {
            aggregation,
            facet: None,
            order,
            _unparsed: false,
        }
    }
    
    pub fn facet(mut self, value: String) -> Self {
        self.facet = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for LogQueryDefinitionGroupBySort {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogQueryDefinitionGroupBySortVisitor;
        impl<'a> Visitor<'a> for LogQueryDefinitionGroupBySortVisitor {
            type Value = LogQueryDefinitionGroupBySort;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut aggregation: Option<crate::datadogV1_20270101::model::EventsAggregator> = None;
                let mut facet: Option<String> = None;
                let mut order: Option<crate::datadogV1_20270101::model::QuerySortOrder> = None;
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
                        "facet" => {
                            if v.is_null() {
                                continue;
                            }
                            facet = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "order" => {
                            order = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _order) = order {
                                match _order {
                                    crate::datadogV1_20270101::model::QuerySortOrder::UnparsedObject(_order) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let aggregation = aggregation.ok_or_else(|| M::Error::missing_field("aggregation"))?;
                let order = order.ok_or_else(|| M::Error::missing_field("order"))?;

                let content = LogQueryDefinitionGroupBySort {
                    aggregation,
                    facet,
                    order,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogQueryDefinitionGroupBySortVisitor)
    }
}

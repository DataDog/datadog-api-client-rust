// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Defines the event that counts as a return, and the window in which it must occur.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductAnalyticsRetentionReturnCriteria {
    /// A query definition discriminated by the `data_source` field.
    /// Use `product_analytics` for standard event queries, or
    /// `product_analytics_occurrence` for occurrence-filtered queries.
    #[serde(rename = "base_query")]
    pub base_query: crate::datadogV2::model::ProductAnalyticsBaseQuery,
    /// A retention interval, either aligned to calendar boundaries or of a fixed length.
    /// Cohort criteria use calendar intervals; return criteria use fixed intervals.
    #[serde(rename = "time_interval")]
    pub time_interval: Option<crate::datadogV2::model::ProductAnalyticsRetentionTimeInterval>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductAnalyticsRetentionReturnCriteria {
    pub fn new(
        base_query: crate::datadogV2::model::ProductAnalyticsBaseQuery,
    ) -> ProductAnalyticsRetentionReturnCriteria {
        ProductAnalyticsRetentionReturnCriteria {
            base_query,
            time_interval: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn time_interval(
        mut self,
        value: crate::datadogV2::model::ProductAnalyticsRetentionTimeInterval,
    ) -> Self {
        self.time_interval = Some(value);
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

impl<'de> Deserialize<'de> for ProductAnalyticsRetentionReturnCriteria {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsRetentionReturnCriteriaVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsRetentionReturnCriteriaVisitor {
            type Value = ProductAnalyticsRetentionReturnCriteria;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut base_query: Option<crate::datadogV2::model::ProductAnalyticsBaseQuery> =
                    None;
                let mut time_interval: Option<
                    crate::datadogV2::model::ProductAnalyticsRetentionTimeInterval,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "base_query" => {
                            base_query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _base_query) = base_query {
                                match _base_query {
                                    crate::datadogV2::model::ProductAnalyticsBaseQuery::UnparsedObject(_base_query) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "time_interval" => {
                            if v.is_null() {
                                continue;
                            }
                            time_interval =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _time_interval) = time_interval {
                                match _time_interval {
                                    crate::datadogV2::model::ProductAnalyticsRetentionTimeInterval::UnparsedObject(_time_interval) => {
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
                let base_query = base_query.ok_or_else(|| M::Error::missing_field("base_query"))?;

                let content = ProductAnalyticsRetentionReturnCriteria {
                    base_query,
                    time_interval,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductAnalyticsRetentionReturnCriteriaVisitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The analytics list query definition. It selects the events to return with `query`, then
/// chooses the columns on each event row, the sort applied to those rows, and a row limit.
/// Unlike the scalar and timeseries queries, a list query returns raw event rows rather than
/// aggregates, so it takes no compute or group-by rule.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductAnalyticsAnalyticsListQuery {
    /// Audience filter definitions for targeting specific user segments.
    #[serde(rename = "audience_filters")]
    pub audience_filters: Option<crate::datadogV2::model::ProductAnalyticsAudienceFilters>,
    /// Attribute columns to include in each event row.
    #[serde(rename = "columns")]
    pub columns: Option<Vec<String>>,
    /// Maximum number of event rows to return.
    #[serde(rename = "limit")]
    pub limit: Option<i64>,
    /// A query definition discriminated by the `data_source` field.
    /// Use `product_analytics` for standard event queries, or
    /// `product_analytics_occurrence` for occurrence-filtered queries.
    #[serde(rename = "query")]
    pub query: crate::datadogV2::model::ProductAnalyticsBaseQuery,
    /// The sort applied to the returned event rows.
    #[serde(rename = "sort")]
    pub sort: Option<crate::datadogV2::model::ProductAnalyticsAnalyticsListSort>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductAnalyticsAnalyticsListQuery {
    pub fn new(
        query: crate::datadogV2::model::ProductAnalyticsBaseQuery,
    ) -> ProductAnalyticsAnalyticsListQuery {
        ProductAnalyticsAnalyticsListQuery {
            audience_filters: None,
            columns: None,
            limit: None,
            query,
            sort: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn audience_filters(
        mut self,
        value: crate::datadogV2::model::ProductAnalyticsAudienceFilters,
    ) -> Self {
        self.audience_filters = Some(value);
        self
    }

    pub fn columns(mut self, value: Vec<String>) -> Self {
        self.columns = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn sort(
        mut self,
        value: crate::datadogV2::model::ProductAnalyticsAnalyticsListSort,
    ) -> Self {
        self.sort = Some(value);
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

impl<'de> Deserialize<'de> for ProductAnalyticsAnalyticsListQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsAnalyticsListQueryVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsAnalyticsListQueryVisitor {
            type Value = ProductAnalyticsAnalyticsListQuery;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut audience_filters: Option<
                    crate::datadogV2::model::ProductAnalyticsAudienceFilters,
                > = None;
                let mut columns: Option<Vec<String>> = None;
                let mut limit: Option<i64> = None;
                let mut query: Option<crate::datadogV2::model::ProductAnalyticsBaseQuery> = None;
                let mut sort: Option<crate::datadogV2::model::ProductAnalyticsAnalyticsListSort> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "audience_filters" => {
                            if v.is_null() {
                                continue;
                            }
                            audience_filters =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "columns" => {
                            if v.is_null() {
                                continue;
                            }
                            columns = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "limit" => {
                            if v.is_null() {
                                continue;
                            }
                            limit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query" => {
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _query) = query {
                                match _query {
                                    crate::datadogV2::model::ProductAnalyticsBaseQuery::UnparsedObject(_query) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "sort" => {
                            if v.is_null() {
                                continue;
                            }
                            sort = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let query = query.ok_or_else(|| M::Error::missing_field("query"))?;

                let content = ProductAnalyticsAnalyticsListQuery {
                    audience_filters,
                    columns,
                    limit,
                    query,
                    sort,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductAnalyticsAnalyticsListQueryVisitor)
    }
}

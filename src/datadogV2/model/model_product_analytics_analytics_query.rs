// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The analytics query definition containing a base query, compute rule, and optional grouping.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductAnalyticsAnalyticsQuery {
    /// Audience filter definitions for targeting specific user segments.
    #[serde(rename = "audience_filters")]
    pub audience_filters: Option<crate::datadogV2::model::ProductAnalyticsAudienceFilters>,
    /// A compute rule for aggregating data.
    #[serde(rename = "compute")]
    pub compute: crate::datadogV2::model::ProductAnalyticsCompute,
    /// Group-by rules for segmenting results.
    #[serde(rename = "group_by")]
    pub group_by: Option<Vec<crate::datadogV2::model::ProductAnalyticsGroupBy>>,
    /// Restrict the query to specific indexes. Max 1 entry.
    #[serde(rename = "indexes")]
    pub indexes: Option<Vec<String>>,
    /// A query definition discriminated by the `data_source` field.
    /// Use `product_analytics` for standard event queries, or
    /// `product_analytics_occurrence` for occurrence-filtered queries.
    #[serde(rename = "query")]
    pub query: crate::datadogV2::model::ProductAnalyticsBaseQuery,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductAnalyticsAnalyticsQuery {
    pub fn new(
        compute: crate::datadogV2::model::ProductAnalyticsCompute,
        query: crate::datadogV2::model::ProductAnalyticsBaseQuery,
    ) -> ProductAnalyticsAnalyticsQuery {
        ProductAnalyticsAnalyticsQuery {
            audience_filters: None,
            compute,
            group_by: None,
            indexes: None,
            query,
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

    pub fn group_by(
        mut self,
        value: Vec<crate::datadogV2::model::ProductAnalyticsGroupBy>,
    ) -> Self {
        self.group_by = Some(value);
        self
    }

    pub fn indexes(mut self, value: Vec<String>) -> Self {
        self.indexes = Some(value);
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

impl<'de> Deserialize<'de> for ProductAnalyticsAnalyticsQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsAnalyticsQueryVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsAnalyticsQueryVisitor {
            type Value = ProductAnalyticsAnalyticsQuery;

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
                let mut compute: Option<crate::datadogV2::model::ProductAnalyticsCompute> = None;
                let mut group_by: Option<Vec<crate::datadogV2::model::ProductAnalyticsGroupBy>> =
                    None;
                let mut indexes: Option<Vec<String>> = None;
                let mut query: Option<crate::datadogV2::model::ProductAnalyticsBaseQuery> = None;
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
                        "compute" => {
                            compute = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "group_by" => {
                            if v.is_null() {
                                continue;
                            }
                            group_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "indexes" => {
                            if v.is_null() {
                                continue;
                            }
                            indexes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let compute = compute.ok_or_else(|| M::Error::missing_field("compute"))?;
                let query = query.ok_or_else(|| M::Error::missing_field("query"))?;

                let content = ProductAnalyticsAnalyticsQuery {
                    audience_filters,
                    compute,
                    group_by,
                    indexes,
                    query,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductAnalyticsAnalyticsQueryVisitor)
    }
}

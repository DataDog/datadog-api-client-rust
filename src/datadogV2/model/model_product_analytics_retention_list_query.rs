// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Query definition for a retention list request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductAnalyticsRetentionListQuery {
    /// The attribute columns to include in each returned row.
    #[serde(rename = "columns")]
    pub columns: Option<Vec<crate::datadogV2::model::ProductAnalyticsRetentionListColumn>>,
    /// Narrows a retention query to a single cell, at the intersection of one cohort and one return period.
    #[serde(rename = "computation_scope")]
    pub computation_scope: crate::datadogV2::model::ProductAnalyticsRetentionCellScope,
    /// Maximum number of rows to return. Use `0` for no limit.
    #[serde(rename = "limit")]
    pub limit: Option<i64>,
    /// Defines the cohort and return criteria that make up a retention query.
    #[serde(rename = "search")]
    pub search: crate::datadogV2::model::ProductAnalyticsRetentionSearch,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductAnalyticsRetentionListQuery {
    pub fn new(
        computation_scope: crate::datadogV2::model::ProductAnalyticsRetentionCellScope,
        search: crate::datadogV2::model::ProductAnalyticsRetentionSearch,
    ) -> ProductAnalyticsRetentionListQuery {
        ProductAnalyticsRetentionListQuery {
            columns: None,
            computation_scope,
            limit: None,
            search,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn columns(
        mut self,
        value: Vec<crate::datadogV2::model::ProductAnalyticsRetentionListColumn>,
    ) -> Self {
        self.columns = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
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

impl<'de> Deserialize<'de> for ProductAnalyticsRetentionListQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsRetentionListQueryVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsRetentionListQueryVisitor {
            type Value = ProductAnalyticsRetentionListQuery;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut columns: Option<
                    Vec<crate::datadogV2::model::ProductAnalyticsRetentionListColumn>,
                > = None;
                let mut computation_scope: Option<
                    crate::datadogV2::model::ProductAnalyticsRetentionCellScope,
                > = None;
                let mut limit: Option<i64> = None;
                let mut search: Option<crate::datadogV2::model::ProductAnalyticsRetentionSearch> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "columns" => {
                            if v.is_null() {
                                continue;
                            }
                            columns = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "computation_scope" => {
                            computation_scope =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "limit" => {
                            if v.is_null() {
                                continue;
                            }
                            limit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "search" => {
                            search = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let computation_scope = computation_scope
                    .ok_or_else(|| M::Error::missing_field("computation_scope"))?;
                let search = search.ok_or_else(|| M::Error::missing_field("search"))?;

                let content = ProductAnalyticsRetentionListQuery {
                    columns,
                    computation_scope,
                    limit,
                    search,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductAnalyticsRetentionListQueryVisitor)
    }
}

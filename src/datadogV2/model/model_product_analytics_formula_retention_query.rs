// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Query definition for a retention scalar or retention timeseries request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductAnalyticsFormulaRetentionQuery {
    /// Restricts a retention query to part of the grid, so that results can be examined in detail.
    /// Omit it to compute the whole grid.
    #[serde(rename = "computation_scope")]
    pub computation_scope: Option<crate::datadogV2::model::ProductAnalyticsRetentionScope>,
    /// The metric and aggregation applied to a retention query.
    #[serde(rename = "compute")]
    pub compute: crate::datadogV2::model::ProductAnalyticsRetentionCompute,
    /// Splits the results by the values of one or more facets.
    #[serde(rename = "group_by")]
    pub group_by: Option<Vec<crate::datadogV2::model::ProductAnalyticsRetentionGroupBy>>,
    /// Defines the cohort and return criteria that make up a retention query.
    #[serde(rename = "search")]
    pub search: crate::datadogV2::model::ProductAnalyticsRetentionSearch,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductAnalyticsFormulaRetentionQuery {
    pub fn new(
        compute: crate::datadogV2::model::ProductAnalyticsRetentionCompute,
        search: crate::datadogV2::model::ProductAnalyticsRetentionSearch,
    ) -> ProductAnalyticsFormulaRetentionQuery {
        ProductAnalyticsFormulaRetentionQuery {
            computation_scope: None,
            compute,
            group_by: None,
            search,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn computation_scope(
        mut self,
        value: crate::datadogV2::model::ProductAnalyticsRetentionScope,
    ) -> Self {
        self.computation_scope = Some(value);
        self
    }

    pub fn group_by(
        mut self,
        value: Vec<crate::datadogV2::model::ProductAnalyticsRetentionGroupBy>,
    ) -> Self {
        self.group_by = Some(value);
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

impl<'de> Deserialize<'de> for ProductAnalyticsFormulaRetentionQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsFormulaRetentionQueryVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsFormulaRetentionQueryVisitor {
            type Value = ProductAnalyticsFormulaRetentionQuery;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut computation_scope: Option<
                    crate::datadogV2::model::ProductAnalyticsRetentionScope,
                > = None;
                let mut compute: Option<crate::datadogV2::model::ProductAnalyticsRetentionCompute> =
                    None;
                let mut group_by: Option<
                    Vec<crate::datadogV2::model::ProductAnalyticsRetentionGroupBy>,
                > = None;
                let mut search: Option<crate::datadogV2::model::ProductAnalyticsRetentionSearch> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "computation_scope" => {
                            if v.is_null() {
                                continue;
                            }
                            computation_scope =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _computation_scope) = computation_scope {
                                match _computation_scope {
                                    crate::datadogV2::model::ProductAnalyticsRetentionScope::UnparsedObject(_computation_scope) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
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
                let compute = compute.ok_or_else(|| M::Error::missing_field("compute"))?;
                let search = search.ok_or_else(|| M::Error::missing_field("search"))?;

                let content = ProductAnalyticsFormulaRetentionQuery {
                    computation_scope,
                    compute,
                    group_by,
                    search,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductAnalyticsFormulaRetentionQueryVisitor)
    }
}

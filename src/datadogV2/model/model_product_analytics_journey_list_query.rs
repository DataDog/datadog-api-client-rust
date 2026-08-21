// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Query definition for a journey list request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductAnalyticsJourneyListQuery {
    /// Computed columns to add to each row.
    #[serde(rename = "computed_columns")]
    pub computed_columns:
        Option<Vec<crate::datadogV2::model::ProductAnalyticsJourneyComputedColumn>>,
    /// Whether to return the entities that converted at the target step, or those that dropped off.
    #[serde(rename = "conversion_type")]
    pub conversion_type: Option<crate::datadogV2::model::ProductAnalyticsJourneyConversionType>,
    /// Attribute columns to return for each row, in addition to the identity join key and `timestamp`.
    #[serde(rename = "entity_columns")]
    pub entity_columns: Option<Vec<String>>,
    /// Additional search query applied to the returned rows.
    #[serde(rename = "entity_filters")]
    pub entity_filters: Option<String>,
    /// Segments the results by the values of one or more facets.
    #[serde(rename = "group_by")]
    pub group_by: Option<Vec<crate::datadogV2::model::ProductAnalyticsGraphQueryGroupBy>>,
    /// Maximum number of rows to return. Omit it to let the service choose.
    #[serde(rename = "limit")]
    pub limit: Option<i64>,
    /// Defines the steps of the journey and the filters applied to it.
    #[serde(rename = "search")]
    pub search: crate::datadogV2::model::ProductAnalyticsJourneySearch,
    /// Sort configuration for the returned rows. The sort is applied only when `facet`
    /// is one of the returned columns; otherwise it is ignored.
    #[serde(rename = "sort")]
    pub sort: Option<crate::datadogV2::model::ProductAnalyticsJourneyListSort>,
    /// A reference to a step, or a range of steps, in the journey.
    /// Use a `node` target to name a single step, or a `path` target to name the range
    /// between two steps.
    #[serde(rename = "target")]
    pub target: Option<crate::datadogV2::model::ProductAnalyticsJourneyTarget>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductAnalyticsJourneyListQuery {
    pub fn new(
        search: crate::datadogV2::model::ProductAnalyticsJourneySearch,
    ) -> ProductAnalyticsJourneyListQuery {
        ProductAnalyticsJourneyListQuery {
            computed_columns: None,
            conversion_type: None,
            entity_columns: None,
            entity_filters: None,
            group_by: None,
            limit: None,
            search,
            sort: None,
            target: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn computed_columns(
        mut self,
        value: Vec<crate::datadogV2::model::ProductAnalyticsJourneyComputedColumn>,
    ) -> Self {
        self.computed_columns = Some(value);
        self
    }

    pub fn conversion_type(
        mut self,
        value: crate::datadogV2::model::ProductAnalyticsJourneyConversionType,
    ) -> Self {
        self.conversion_type = Some(value);
        self
    }

    pub fn entity_columns(mut self, value: Vec<String>) -> Self {
        self.entity_columns = Some(value);
        self
    }

    pub fn entity_filters(mut self, value: String) -> Self {
        self.entity_filters = Some(value);
        self
    }

    pub fn group_by(
        mut self,
        value: Vec<crate::datadogV2::model::ProductAnalyticsGraphQueryGroupBy>,
    ) -> Self {
        self.group_by = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn sort(mut self, value: crate::datadogV2::model::ProductAnalyticsJourneyListSort) -> Self {
        self.sort = Some(value);
        self
    }

    pub fn target(mut self, value: crate::datadogV2::model::ProductAnalyticsJourneyTarget) -> Self {
        self.target = Some(value);
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

impl<'de> Deserialize<'de> for ProductAnalyticsJourneyListQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsJourneyListQueryVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsJourneyListQueryVisitor {
            type Value = ProductAnalyticsJourneyListQuery;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut computed_columns: Option<
                    Vec<crate::datadogV2::model::ProductAnalyticsJourneyComputedColumn>,
                > = None;
                let mut conversion_type: Option<
                    crate::datadogV2::model::ProductAnalyticsJourneyConversionType,
                > = None;
                let mut entity_columns: Option<Vec<String>> = None;
                let mut entity_filters: Option<String> = None;
                let mut group_by: Option<
                    Vec<crate::datadogV2::model::ProductAnalyticsGraphQueryGroupBy>,
                > = None;
                let mut limit: Option<i64> = None;
                let mut search: Option<crate::datadogV2::model::ProductAnalyticsJourneySearch> =
                    None;
                let mut sort: Option<crate::datadogV2::model::ProductAnalyticsJourneyListSort> =
                    None;
                let mut target: Option<crate::datadogV2::model::ProductAnalyticsJourneyTarget> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "computed_columns" => {
                            if v.is_null() {
                                continue;
                            }
                            computed_columns =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "conversion_type" => {
                            if v.is_null() {
                                continue;
                            }
                            conversion_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _conversion_type) = conversion_type {
                                match _conversion_type {
                                    crate::datadogV2::model::ProductAnalyticsJourneyConversionType::UnparsedObject(_conversion_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "entity_columns" => {
                            if v.is_null() {
                                continue;
                            }
                            entity_columns =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "entity_filters" => {
                            if v.is_null() {
                                continue;
                            }
                            entity_filters =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "group_by" => {
                            if v.is_null() {
                                continue;
                            }
                            group_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "sort" => {
                            if v.is_null() {
                                continue;
                            }
                            sort = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "target" => {
                            if v.is_null() {
                                continue;
                            }
                            target = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _target) = target {
                                match _target {
                                    crate::datadogV2::model::ProductAnalyticsJourneyTarget::UnparsedObject(_target) => {
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
                let search = search.ok_or_else(|| M::Error::missing_field("search"))?;

                let content = ProductAnalyticsJourneyListQuery {
                    computed_columns,
                    conversion_type,
                    entity_columns,
                    entity_filters,
                    group_by,
                    limit,
                    search,
                    sort,
                    target,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductAnalyticsJourneyListQueryVisitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Segments journey results by the values of a facet.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductAnalyticsGraphQueryGroupBy {
    /// Attribute path to group by.
    #[serde(rename = "facet")]
    pub facet: String,
    /// Maximum number of groups to return. Omit it to let the service choose.
    #[serde(rename = "limit")]
    pub limit: Option<i64>,
    /// Whether to exclude entities that have no value for this facet.
    #[serde(rename = "should_exclude_missing")]
    pub should_exclude_missing: Option<bool>,
    /// Sort configuration for group-by results.
    #[serde(rename = "sort")]
    pub sort: Option<crate::datadogV2::model::ProductAnalyticsGroupBySort>,
    /// Audience dimension to group by, instead of an event facet.
    #[serde(rename = "source")]
    pub source: Option<crate::datadogV2::model::ProductAnalyticsGraphQueryGroupBySource>,
    /// A reference to a step, or a range of steps, in the journey.
    /// Use a `node` target to name a single step, or a `path` target to name the range
    /// between two steps.
    #[serde(rename = "target")]
    pub target: Option<crate::datadogV2::model::ProductAnalyticsJourneyTarget>,
    /// Restricts the results to these facet values.
    #[serde(rename = "value_filters")]
    pub value_filters: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductAnalyticsGraphQueryGroupBy {
    pub fn new(facet: String) -> ProductAnalyticsGraphQueryGroupBy {
        ProductAnalyticsGraphQueryGroupBy {
            facet,
            limit: None,
            should_exclude_missing: None,
            sort: None,
            source: None,
            target: None,
            value_filters: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn should_exclude_missing(mut self, value: bool) -> Self {
        self.should_exclude_missing = Some(value);
        self
    }

    pub fn sort(mut self, value: crate::datadogV2::model::ProductAnalyticsGroupBySort) -> Self {
        self.sort = Some(value);
        self
    }

    pub fn source(
        mut self,
        value: crate::datadogV2::model::ProductAnalyticsGraphQueryGroupBySource,
    ) -> Self {
        self.source = Some(value);
        self
    }

    pub fn target(mut self, value: crate::datadogV2::model::ProductAnalyticsJourneyTarget) -> Self {
        self.target = Some(value);
        self
    }

    pub fn value_filters(mut self, value: Vec<String>) -> Self {
        self.value_filters = Some(value);
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

impl<'de> Deserialize<'de> for ProductAnalyticsGraphQueryGroupBy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsGraphQueryGroupByVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsGraphQueryGroupByVisitor {
            type Value = ProductAnalyticsGraphQueryGroupBy;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut facet: Option<String> = None;
                let mut limit: Option<i64> = None;
                let mut should_exclude_missing: Option<bool> = None;
                let mut sort: Option<crate::datadogV2::model::ProductAnalyticsGroupBySort> = None;
                let mut source: Option<
                    crate::datadogV2::model::ProductAnalyticsGraphQueryGroupBySource,
                > = None;
                let mut target: Option<crate::datadogV2::model::ProductAnalyticsJourneyTarget> =
                    None;
                let mut value_filters: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "facet" => {
                            facet = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "limit" => {
                            if v.is_null() {
                                continue;
                            }
                            limit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "should_exclude_missing" => {
                            if v.is_null() {
                                continue;
                            }
                            should_exclude_missing =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sort" => {
                            if v.is_null() {
                                continue;
                            }
                            sort = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source" => {
                            if v.is_null() {
                                continue;
                            }
                            source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _source) = source {
                                match _source {
                                    crate::datadogV2::model::ProductAnalyticsGraphQueryGroupBySource::UnparsedObject(_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
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
                        "value_filters" => {
                            if v.is_null() {
                                continue;
                            }
                            value_filters =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let facet = facet.ok_or_else(|| M::Error::missing_field("facet"))?;

                let content = ProductAnalyticsGraphQueryGroupBy {
                    facet,
                    limit,
                    should_exclude_missing,
                    sort,
                    source,
                    target,
                    value_filters,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductAnalyticsGraphQueryGroupByVisitor)
    }
}

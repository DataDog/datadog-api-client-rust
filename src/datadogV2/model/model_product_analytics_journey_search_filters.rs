// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Filters applied on top of the journey step expression.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductAnalyticsJourneySearchFilters {
    /// Restricts the journey to an audience built from named sub-queries.
    /// Sub-query names must be unique across `users`, `segments`, and `accounts`.
    #[serde(rename = "audience_filters")]
    pub audience_filters: Option<crate::datadogV2::model::ProductAnalyticsJourneyAudienceFilters>,
    /// Filters on journey-level metrics such as time to convert.
    #[serde(rename = "graph_filters")]
    pub graph_filters:
        Option<Vec<crate::datadogV2::model::ProductAnalyticsJourneySearchGraphFilter>>,
    /// Free-text search query applied to the whole journey.
    #[serde(rename = "string_filter")]
    pub string_filter: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductAnalyticsJourneySearchFilters {
    pub fn new() -> ProductAnalyticsJourneySearchFilters {
        ProductAnalyticsJourneySearchFilters {
            audience_filters: None,
            graph_filters: None,
            string_filter: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn audience_filters(
        mut self,
        value: crate::datadogV2::model::ProductAnalyticsJourneyAudienceFilters,
    ) -> Self {
        self.audience_filters = Some(value);
        self
    }

    pub fn graph_filters(
        mut self,
        value: Vec<crate::datadogV2::model::ProductAnalyticsJourneySearchGraphFilter>,
    ) -> Self {
        self.graph_filters = Some(value);
        self
    }

    pub fn string_filter(mut self, value: String) -> Self {
        self.string_filter = Some(value);
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

impl Default for ProductAnalyticsJourneySearchFilters {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ProductAnalyticsJourneySearchFilters {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsJourneySearchFiltersVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsJourneySearchFiltersVisitor {
            type Value = ProductAnalyticsJourneySearchFilters;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut audience_filters: Option<
                    crate::datadogV2::model::ProductAnalyticsJourneyAudienceFilters,
                > = None;
                let mut graph_filters: Option<
                    Vec<crate::datadogV2::model::ProductAnalyticsJourneySearchGraphFilter>,
                > = None;
                let mut string_filter: Option<String> = None;
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
                        "graph_filters" => {
                            if v.is_null() {
                                continue;
                            }
                            graph_filters =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "string_filter" => {
                            if v.is_null() {
                                continue;
                            }
                            string_filter =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ProductAnalyticsJourneySearchFilters {
                    audience_filters,
                    graph_filters,
                    string_filter,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductAnalyticsJourneySearchFiltersVisitor)
    }
}

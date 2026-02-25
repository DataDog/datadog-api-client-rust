// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Search parameters for a Sankey query.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductAnalyticsSankeySearch {
    /// Audience filter definitions for targeting specific user segments.
    #[serde(rename = "audience_filters")]
    pub audience_filters: Option<crate::datadogV2::model::ProductAnalyticsAudienceFilters>,
    /// Join key configuration for correlating events.
    #[serde(rename = "join_keys")]
    pub join_keys: Option<crate::datadogV2::model::ProductAnalyticsJoinKeys>,
    /// Filter for occurrence-based queries.
    #[serde(rename = "occurrences")]
    pub occurrences: Option<crate::datadogV2::model::ProductAnalyticsOccurrenceFilter>,
    /// The search query. Cannot be empty.
    #[serde(rename = "query")]
    pub query: String,
    #[serde(rename = "subquery_id")]
    pub subquery_id: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductAnalyticsSankeySearch {
    pub fn new(query: String) -> ProductAnalyticsSankeySearch {
        ProductAnalyticsSankeySearch {
            audience_filters: None,
            join_keys: None,
            occurrences: None,
            query,
            subquery_id: None,
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

    pub fn join_keys(mut self, value: crate::datadogV2::model::ProductAnalyticsJoinKeys) -> Self {
        self.join_keys = Some(value);
        self
    }

    pub fn occurrences(
        mut self,
        value: crate::datadogV2::model::ProductAnalyticsOccurrenceFilter,
    ) -> Self {
        self.occurrences = Some(value);
        self
    }

    pub fn subquery_id(mut self, value: String) -> Self {
        self.subquery_id = Some(value);
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

impl<'de> Deserialize<'de> for ProductAnalyticsSankeySearch {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsSankeySearchVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsSankeySearchVisitor {
            type Value = ProductAnalyticsSankeySearch;

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
                let mut join_keys: Option<crate::datadogV2::model::ProductAnalyticsJoinKeys> = None;
                let mut occurrences: Option<
                    crate::datadogV2::model::ProductAnalyticsOccurrenceFilter,
                > = None;
                let mut query: Option<String> = None;
                let mut subquery_id: Option<String> = None;
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
                        "join_keys" => {
                            if v.is_null() {
                                continue;
                            }
                            join_keys = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "occurrences" => {
                            if v.is_null() {
                                continue;
                            }
                            occurrences =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query" => {
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "subquery_id" => {
                            if v.is_null() {
                                continue;
                            }
                            subquery_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let query = query.ok_or_else(|| M::Error::missing_field("query"))?;

                let content = ProductAnalyticsSankeySearch {
                    audience_filters,
                    join_keys,
                    occurrences,
                    query,
                    subquery_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductAnalyticsSankeySearchVisitor)
    }
}

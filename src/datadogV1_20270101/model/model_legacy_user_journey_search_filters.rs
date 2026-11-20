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
pub struct LegacyUserJourneySearchFilters {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "audience_filters")]
    pub audience_filters: Option<crate::datadogV1_20270101::model::AudienceFilters>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "graph_filters")]
    pub graph_filters: Option<Vec<crate::datadogV1_20270101::model::GraphQuerySearchFilter>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "string_filter")]
    pub string_filter: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl LegacyUserJourneySearchFilters {
    pub fn new() -> LegacyUserJourneySearchFilters {
        LegacyUserJourneySearchFilters {
            audience_filters: None,
            graph_filters: None,
            string_filter: None,
            _unparsed: false,
        }
    }
    
    pub fn audience_filters(mut self, value: crate::datadogV1_20270101::model::AudienceFilters) -> Self {
        self.audience_filters = Some(value);
        self
    }
    
    pub fn graph_filters(mut self, value: Vec<crate::datadogV1_20270101::model::GraphQuerySearchFilter>) -> Self {
        self.graph_filters = Some(value);
        self
    }
    
    pub fn string_filter(mut self, value: String) -> Self {
        self.string_filter = Some(value);
        self
    }
    
}


impl Default for LegacyUserJourneySearchFilters {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for LegacyUserJourneySearchFilters {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LegacyUserJourneySearchFiltersVisitor;
        impl<'a> Visitor<'a> for LegacyUserJourneySearchFiltersVisitor {
            type Value = LegacyUserJourneySearchFilters;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut audience_filters: Option<crate::datadogV1_20270101::model::AudienceFilters> = None;
                let mut graph_filters: Option<Vec<crate::datadogV1_20270101::model::GraphQuerySearchFilter>> = None;
                let mut string_filter: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "audience_filters" => {
                            if v.is_null() {
                                continue;
                            }
                            audience_filters = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "graph_filters" => {
                            if v.is_null() {
                                continue;
                            }
                            graph_filters = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "string_filter" => {
                            if v.is_null() {
                                continue;
                            }
                            string_filter = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }

                let content = LegacyUserJourneySearchFilters {
                    audience_filters,
                    graph_filters,
                    string_filter,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LegacyUserJourneySearchFiltersVisitor)
    }
}

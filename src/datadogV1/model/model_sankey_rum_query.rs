// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Sankey widget with RUM data source query.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SankeyRumQuery {
    /// Product Analytics/RUM audience filters.
    #[serde(rename = "audience_filters")]
    pub audience_filters: Option<crate::datadogV1::model::ProductAnalyticsAudienceFilters>,
    /// Sankey widget with RUM data source.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1::model::SankeyRumDataSource,
    /// Entries per step.
    #[serde(rename = "entries_per_step")]
    pub entries_per_step: Option<i64>,
    /// Join keys.
    #[serde(rename = "join_keys")]
    pub join_keys: Option<crate::datadogV1::model::SankeyJoinKeys>,
    /// Sankey mode for RUM queries.
    #[serde(rename = "mode")]
    pub mode: crate::datadogV1::model::SankeyRumQueryMode,
    /// Number of steps.
    #[serde(rename = "number_of_steps")]
    pub number_of_steps: Option<i64>,
    #[serde(rename = "occurrences")]
    pub occurrences: Option<crate::datadogV1::model::ProductAnalyticsAudienceOccurrenceFilter>,
    /// Query string.
    #[serde(rename = "query_string")]
    pub query_string: String,
    /// Source.
    #[serde(rename = "source")]
    pub source: Option<String>,
    /// Subquery ID.
    #[serde(rename = "subquery_id")]
    pub subquery_id: Option<String>,
    /// Target.
    #[serde(rename = "target")]
    pub target: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SankeyRumQuery {
    pub fn new(
        data_source: crate::datadogV1::model::SankeyRumDataSource,
        mode: crate::datadogV1::model::SankeyRumQueryMode,
        query_string: String,
    ) -> SankeyRumQuery {
        SankeyRumQuery {
            audience_filters: None,
            data_source,
            entries_per_step: None,
            join_keys: None,
            mode,
            number_of_steps: None,
            occurrences: None,
            query_string,
            source: None,
            subquery_id: None,
            target: None,
            _unparsed: false,
        }
    }

    pub fn audience_filters(
        mut self,
        value: crate::datadogV1::model::ProductAnalyticsAudienceFilters,
    ) -> Self {
        self.audience_filters = Some(value);
        self
    }

    pub fn entries_per_step(mut self, value: i64) -> Self {
        self.entries_per_step = Some(value);
        self
    }

    pub fn join_keys(mut self, value: crate::datadogV1::model::SankeyJoinKeys) -> Self {
        self.join_keys = Some(value);
        self
    }

    pub fn number_of_steps(mut self, value: i64) -> Self {
        self.number_of_steps = Some(value);
        self
    }

    pub fn occurrences(
        mut self,
        value: crate::datadogV1::model::ProductAnalyticsAudienceOccurrenceFilter,
    ) -> Self {
        self.occurrences = Some(value);
        self
    }

    pub fn source(mut self, value: String) -> Self {
        self.source = Some(value);
        self
    }

    pub fn subquery_id(mut self, value: String) -> Self {
        self.subquery_id = Some(value);
        self
    }

    pub fn target(mut self, value: String) -> Self {
        self.target = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for SankeyRumQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SankeyRumQueryVisitor;
        impl<'a> Visitor<'a> for SankeyRumQueryVisitor {
            type Value = SankeyRumQuery;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut audience_filters: Option<
                    crate::datadogV1::model::ProductAnalyticsAudienceFilters,
                > = None;
                let mut data_source: Option<crate::datadogV1::model::SankeyRumDataSource> = None;
                let mut entries_per_step: Option<i64> = None;
                let mut join_keys: Option<crate::datadogV1::model::SankeyJoinKeys> = None;
                let mut mode: Option<crate::datadogV1::model::SankeyRumQueryMode> = None;
                let mut number_of_steps: Option<i64> = None;
                let mut occurrences: Option<
                    crate::datadogV1::model::ProductAnalyticsAudienceOccurrenceFilter,
                > = None;
                let mut query_string: Option<String> = None;
                let mut source: Option<String> = None;
                let mut subquery_id: Option<String> = None;
                let mut target: Option<String> = None;
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
                        "data_source" => {
                            data_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _data_source) = data_source {
                                match _data_source {
                                    crate::datadogV1::model::SankeyRumDataSource::UnparsedObject(_data_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "entries_per_step" => {
                            if v.is_null() {
                                continue;
                            }
                            entries_per_step =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "join_keys" => {
                            if v.is_null() {
                                continue;
                            }
                            join_keys = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mode" => {
                            mode = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _mode) = mode {
                                match _mode {
                                    crate::datadogV1::model::SankeyRumQueryMode::UnparsedObject(
                                        _mode,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "number_of_steps" => {
                            if v.is_null() {
                                continue;
                            }
                            number_of_steps =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "occurrences" => {
                            if v.is_null() {
                                continue;
                            }
                            occurrences =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query_string" => {
                            query_string =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source" => {
                            if v.is_null() {
                                continue;
                            }
                            source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "subquery_id" => {
                            if v.is_null() {
                                continue;
                            }
                            subquery_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "target" => {
                            if v.is_null() {
                                continue;
                            }
                            target = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let data_source =
                    data_source.ok_or_else(|| M::Error::missing_field("data_source"))?;
                let mode = mode.ok_or_else(|| M::Error::missing_field("mode"))?;
                let query_string =
                    query_string.ok_or_else(|| M::Error::missing_field("query_string"))?;

                let content = SankeyRumQuery {
                    audience_filters,
                    data_source,
                    entries_per_step,
                    join_keys,
                    mode,
                    number_of_steps,
                    occurrences,
                    query_string,
                    source,
                    subquery_id,
                    target,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SankeyRumQueryVisitor)
    }
}

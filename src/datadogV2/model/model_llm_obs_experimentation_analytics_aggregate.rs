// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Analytics aggregation parameters.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsExperimentationAnalyticsAggregate {
    /// List of metric computations to perform.
    #[serde(rename = "compute")]
    pub compute: Vec<crate::datadogV2::model::LLMObsExperimentationAnalyticsCompute>,
    /// Filter to a specific dataset version.
    #[serde(
        rename = "dataset_version",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub dataset_version: Option<Option<i64>>,
    /// Fields to group results by.
    #[serde(rename = "group_by")]
    pub group_by: Option<Vec<crate::datadogV2::model::LLMObsExperimentationAnalyticsGroupBy>>,
    /// Data indexes to query. At least one is required.
    #[serde(rename = "indexes")]
    pub indexes: Vec<String>,
    /// Maximum number of results to return.
    #[serde(rename = "limit", default, with = "::serde_with::rust::double_option")]
    pub limit: Option<Option<i32>>,
    /// Search query for filtering analytics data.
    #[serde(rename = "search")]
    pub search: crate::datadogV2::model::LLMObsExperimentationAnalyticsSearch,
    /// Unix-millisecond time range for filtering analytics data.
    #[serde(rename = "time")]
    pub time: Option<crate::datadogV2::model::LLMObsExperimentationAnalyticsTimeRange>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsExperimentationAnalyticsAggregate {
    pub fn new(
        compute: Vec<crate::datadogV2::model::LLMObsExperimentationAnalyticsCompute>,
        indexes: Vec<String>,
        search: crate::datadogV2::model::LLMObsExperimentationAnalyticsSearch,
    ) -> LLMObsExperimentationAnalyticsAggregate {
        LLMObsExperimentationAnalyticsAggregate {
            compute,
            dataset_version: None,
            group_by: None,
            indexes,
            limit: None,
            search,
            time: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn dataset_version(mut self, value: Option<i64>) -> Self {
        self.dataset_version = Some(value);
        self
    }

    pub fn group_by(
        mut self,
        value: Vec<crate::datadogV2::model::LLMObsExperimentationAnalyticsGroupBy>,
    ) -> Self {
        self.group_by = Some(value);
        self
    }

    pub fn limit(mut self, value: Option<i32>) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn time(
        mut self,
        value: crate::datadogV2::model::LLMObsExperimentationAnalyticsTimeRange,
    ) -> Self {
        self.time = Some(value);
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

impl<'de> Deserialize<'de> for LLMObsExperimentationAnalyticsAggregate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsExperimentationAnalyticsAggregateVisitor;
        impl<'a> Visitor<'a> for LLMObsExperimentationAnalyticsAggregateVisitor {
            type Value = LLMObsExperimentationAnalyticsAggregate;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut compute: Option<
                    Vec<crate::datadogV2::model::LLMObsExperimentationAnalyticsCompute>,
                > = None;
                let mut dataset_version: Option<Option<i64>> = None;
                let mut group_by: Option<
                    Vec<crate::datadogV2::model::LLMObsExperimentationAnalyticsGroupBy>,
                > = None;
                let mut indexes: Option<Vec<String>> = None;
                let mut limit: Option<Option<i32>> = None;
                let mut search: Option<
                    crate::datadogV2::model::LLMObsExperimentationAnalyticsSearch,
                > = None;
                let mut time: Option<
                    crate::datadogV2::model::LLMObsExperimentationAnalyticsTimeRange,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "compute" => {
                            compute = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "dataset_version" => {
                            dataset_version =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "group_by" => {
                            if v.is_null() {
                                continue;
                            }
                            group_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "indexes" => {
                            indexes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "limit" => {
                            limit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "search" => {
                            search = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "time" => {
                            if v.is_null() {
                                continue;
                            }
                            time = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let compute = compute.ok_or_else(|| M::Error::missing_field("compute"))?;
                let indexes = indexes.ok_or_else(|| M::Error::missing_field("indexes"))?;
                let search = search.ok_or_else(|| M::Error::missing_field("search"))?;

                let content = LLMObsExperimentationAnalyticsAggregate {
                    compute,
                    dataset_version,
                    group_by,
                    indexes,
                    limit,
                    search,
                    time,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsExperimentationAnalyticsAggregateVisitor)
    }
}

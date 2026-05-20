// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Filter criteria for an experimentation search request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsExperimentationFilter {
    /// When `true`, include soft-deleted entities alongside active ones.
    #[serde(rename = "include_deleted")]
    pub include_deleted: Option<bool>,
    /// When `true`, return only soft-deleted entities.
    #[serde(rename = "is_deleted")]
    pub is_deleted: Option<bool>,
    /// Free-text search query.
    #[serde(rename = "query")]
    pub query: Option<String>,
    /// Entity types to search. Valid values are `projects`, `datasets`, `dataset_records`, `experiments`, and `experiment_runs`.
    #[serde(rename = "scope")]
    pub scope: Vec<String>,
    /// Filter dataset records by a specific dataset version.
    #[serde(
        rename = "version",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub version: Option<Option<i64>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsExperimentationFilter {
    pub fn new(scope: Vec<String>) -> LLMObsExperimentationFilter {
        LLMObsExperimentationFilter {
            include_deleted: None,
            is_deleted: None,
            query: None,
            scope,
            version: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn include_deleted(mut self, value: bool) -> Self {
        self.include_deleted = Some(value);
        self
    }

    pub fn is_deleted(mut self, value: bool) -> Self {
        self.is_deleted = Some(value);
        self
    }

    pub fn query(mut self, value: String) -> Self {
        self.query = Some(value);
        self
    }

    pub fn version(mut self, value: Option<i64>) -> Self {
        self.version = Some(value);
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

impl<'de> Deserialize<'de> for LLMObsExperimentationFilter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsExperimentationFilterVisitor;
        impl<'a> Visitor<'a> for LLMObsExperimentationFilterVisitor {
            type Value = LLMObsExperimentationFilter;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut include_deleted: Option<bool> = None;
                let mut is_deleted: Option<bool> = None;
                let mut query: Option<String> = None;
                let mut scope: Option<Vec<String>> = None;
                let mut version: Option<Option<i64>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "include_deleted" => {
                            if v.is_null() {
                                continue;
                            }
                            include_deleted =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_deleted" => {
                            if v.is_null() {
                                continue;
                            }
                            is_deleted = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query" => {
                            if v.is_null() {
                                continue;
                            }
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "scope" => {
                            scope = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "version" => {
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let scope = scope.ok_or_else(|| M::Error::missing_field("scope"))?;

                let content = LLMObsExperimentationFilter {
                    include_deleted,
                    is_deleted,
                    query,
                    scope,
                    version,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsExperimentationFilterVisitor)
    }
}

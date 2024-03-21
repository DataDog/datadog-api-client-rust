// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The log query.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LogQueryDefinition {
    /// Define computation for a log query.
    #[serde(rename = "compute")]
    pub compute: Option<crate::datadogV1::model::LogsQueryCompute>,
    /// List of tag prefixes to group by in the case of a cluster check.
    #[serde(rename = "group_by")]
    pub group_by: Option<Vec<crate::datadogV1::model::LogQueryDefinitionGroupBy>>,
    /// A coma separated-list of index names. Use "*" query all indexes at once. [Multiple Indexes](<https://docs.datadoghq.com/logs/indexes/#multiple-indexes>)
    #[serde(rename = "index")]
    pub index: Option<String>,
    /// This field is mutually exclusive with `compute`.
    #[serde(rename = "multi_compute")]
    pub multi_compute: Option<Vec<crate::datadogV1::model::LogsQueryCompute>>,
    /// The query being made on the logs.
    #[serde(rename = "search")]
    pub search: Option<crate::datadogV1::model::LogQueryDefinitionSearch>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LogQueryDefinition {
    pub fn new() -> LogQueryDefinition {
        LogQueryDefinition {
            compute: None,
            group_by: None,
            index: None,
            multi_compute: None,
            search: None,
            _unparsed: false,
        }
    }

    pub fn compute(mut self, value: crate::datadogV1::model::LogsQueryCompute) -> Self {
        self.compute = Some(value);
        self
    }

    pub fn group_by(
        mut self,
        value: Vec<crate::datadogV1::model::LogQueryDefinitionGroupBy>,
    ) -> Self {
        self.group_by = Some(value);
        self
    }

    pub fn index(mut self, value: String) -> Self {
        self.index = Some(value);
        self
    }

    pub fn multi_compute(mut self, value: Vec<crate::datadogV1::model::LogsQueryCompute>) -> Self {
        self.multi_compute = Some(value);
        self
    }

    pub fn search(mut self, value: crate::datadogV1::model::LogQueryDefinitionSearch) -> Self {
        self.search = Some(value);
        self
    }
}

impl Default for LogQueryDefinition {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for LogQueryDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogQueryDefinitionVisitor;
        impl<'a> Visitor<'a> for LogQueryDefinitionVisitor {
            type Value = LogQueryDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut compute: Option<crate::datadogV1::model::LogsQueryCompute> = None;
                let mut group_by: Option<Vec<crate::datadogV1::model::LogQueryDefinitionGroupBy>> =
                    None;
                let mut index: Option<String> = None;
                let mut multi_compute: Option<Vec<crate::datadogV1::model::LogsQueryCompute>> =
                    None;
                let mut search: Option<crate::datadogV1::model::LogQueryDefinitionSearch> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "compute" => {
                            if v.is_null() {
                                continue;
                            }
                            compute = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "group_by" => {
                            if v.is_null() {
                                continue;
                            }
                            group_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "index" => {
                            if v.is_null() {
                                continue;
                            }
                            index = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "multi_compute" => {
                            if v.is_null() {
                                continue;
                            }
                            multi_compute =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "search" => {
                            if v.is_null() {
                                continue;
                            }
                            search = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = LogQueryDefinition {
                    compute,
                    group_by,
                    index,
                    multi_compute,
                    search,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogQueryDefinitionVisitor)
    }
}

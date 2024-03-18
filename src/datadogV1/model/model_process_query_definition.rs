// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The process query to use in the widget.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProcessQueryDefinition {
    /// List of processes.
    #[serde(rename = "filter_by")]
    pub filter_by: Option<Vec<String>>,
    /// Max number of items in the filter list.
    #[serde(rename = "limit")]
    pub limit: Option<i64>,
    /// Your chosen metric.
    #[serde(rename = "metric")]
    pub metric: String,
    /// Your chosen search term.
    #[serde(rename = "search_by")]
    pub search_by: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProcessQueryDefinition {
    pub fn new(metric: String) -> ProcessQueryDefinition {
        ProcessQueryDefinition {
            filter_by: None,
            limit: None,
            metric,
            search_by: None,
            _unparsed: false,
        }
    }

    pub fn filter_by(mut self, value: Vec<String>) -> Self {
        self.filter_by = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn search_by(mut self, value: String) -> Self {
        self.search_by = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for ProcessQueryDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProcessQueryDefinitionVisitor;
        impl<'a> Visitor<'a> for ProcessQueryDefinitionVisitor {
            type Value = ProcessQueryDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut filter_by: Option<Vec<String>> = None;
                let mut limit: Option<i64> = None;
                let mut metric: Option<String> = None;
                let mut search_by: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "filter_by" => {
                            if v.is_null() {
                                continue;
                            }
                            filter_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "limit" => {
                            if v.is_null() {
                                continue;
                            }
                            limit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metric" => {
                            metric = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "search_by" => {
                            if v.is_null() {
                                continue;
                            }
                            search_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let metric = metric.ok_or_else(|| M::Error::missing_field("metric"))?;

                let content = ProcessQueryDefinition {
                    filter_by,
                    limit,
                    metric,
                    search_by,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProcessQueryDefinitionVisitor)
    }
}

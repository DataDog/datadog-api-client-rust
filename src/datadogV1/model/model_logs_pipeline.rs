// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Pipelines and processors operate on incoming logs,
/// parsing and transforming them into structured attributes for easier querying.
///
/// **Note**: These endpoints are only available for admin users.
/// Make sure to use an application key created by an admin.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LogsPipeline {
    /// Filter for logs.
    #[serde(rename = "filter")]
    pub filter: Option<crate::datadogV1::model::LogsFilter>,
    /// ID of the pipeline.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Whether or not the pipeline is enabled.
    #[serde(rename = "is_enabled")]
    pub is_enabled: Option<bool>,
    /// Whether or not the pipeline can be edited.
    #[serde(rename = "is_read_only")]
    pub is_read_only: Option<bool>,
    /// Name of the pipeline.
    #[serde(rename = "name")]
    pub name: String,
    /// Ordered list of processors in this pipeline.
    #[serde(rename = "processors")]
    pub processors: Option<Vec<crate::datadogV1::model::LogsProcessor>>,
    /// Type of pipeline.
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LogsPipeline {
    pub fn new(name: String) -> LogsPipeline {
        LogsPipeline {
            filter: None,
            id: None,
            is_enabled: None,
            is_read_only: None,
            name,
            processors: None,
            type_: None,
            _unparsed: false,
        }
    }

    pub fn filter(mut self, value: crate::datadogV1::model::LogsFilter) -> Self {
        self.filter = Some(value);
        self
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }

    pub fn is_enabled(mut self, value: bool) -> Self {
        self.is_enabled = Some(value);
        self
    }

    pub fn is_read_only(mut self, value: bool) -> Self {
        self.is_read_only = Some(value);
        self
    }

    pub fn processors(mut self, value: Vec<crate::datadogV1::model::LogsProcessor>) -> Self {
        self.processors = Some(value);
        self
    }

    pub fn type_(mut self, value: String) -> Self {
        self.type_ = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for LogsPipeline {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogsPipelineVisitor;
        impl<'a> Visitor<'a> for LogsPipelineVisitor {
            type Value = LogsPipeline;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut filter: Option<crate::datadogV1::model::LogsFilter> = None;
                let mut id: Option<String> = None;
                let mut is_enabled: Option<bool> = None;
                let mut is_read_only: Option<bool> = None;
                let mut name: Option<String> = None;
                let mut processors: Option<Vec<crate::datadogV1::model::LogsProcessor>> = None;
                let mut type_: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "filter" => {
                            if v.is_null() {
                                continue;
                            }
                            filter = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            is_enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_read_only" => {
                            if v.is_null() {
                                continue;
                            }
                            is_read_only =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "processors" => {
                            if v.is_null() {
                                continue;
                            }
                            processors = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = LogsPipeline {
                    filter,
                    id,
                    is_enabled,
                    is_read_only,
                    name,
                    processors,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogsPipelineVisitor)
    }
}

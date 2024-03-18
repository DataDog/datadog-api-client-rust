// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// There are two ways to improve correlation between application traces and logs.
///
///   1. Follow the documentation on [how to inject a trace ID in the application logs](<https://docs.datadoghq.com/tracing/connect_logs_and_traces>)
///   and by default log integrations take care of all the rest of the setup.
///
///   2. Use the Trace remapper processor to define a log attribute as its associated trace ID.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LogsTraceRemapper {
    /// Whether or not the processor is enabled.
    #[serde(rename = "is_enabled")]
    pub is_enabled: Option<bool>,
    /// Name of the processor.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Array of source attributes.
    #[serde(rename = "sources")]
    pub sources: Option<Vec<String>>,
    /// Type of logs trace remapper.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::LogsTraceRemapperType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LogsTraceRemapper {
    pub fn new(type_: crate::datadogV1::model::LogsTraceRemapperType) -> LogsTraceRemapper {
        LogsTraceRemapper {
            is_enabled: None,
            name: None,
            sources: None,
            type_,
            _unparsed: false,
        }
    }

    pub fn is_enabled(mut self, value: bool) -> Self {
        self.is_enabled = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn sources(mut self, value: Vec<String>) -> Self {
        self.sources = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for LogsTraceRemapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogsTraceRemapperVisitor;
        impl<'a> Visitor<'a> for LogsTraceRemapperVisitor {
            type Value = LogsTraceRemapper;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut is_enabled: Option<bool> = None;
                let mut name: Option<String> = None;
                let mut sources: Option<Vec<String>> = None;
                let mut type_: Option<crate::datadogV1::model::LogsTraceRemapperType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "is_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            is_enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sources" => {
                            if v.is_null() {
                                continue;
                            }
                            sources = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::LogsTraceRemapperType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = LogsTraceRemapper {
                    is_enabled,
                    name,
                    sources,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogsTraceRemapperVisitor)
    }
}

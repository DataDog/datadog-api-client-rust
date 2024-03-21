// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// As Datadog receives logs, it timestamps them using the value(s) from any of these default attributes.
///
///   - `timestamp`
///   - `date`
///   - `_timestamp`
///   - `Timestamp`
///   - `eventTime`
///   - `published_date`
///
///   If your logs put their dates in an attribute not in this list,
///   use the log date Remapper Processor to define their date attribute as the official log timestamp.
///   The recognized date formats are ISO8601, UNIX (the milliseconds EPOCH format), and RFC3164.
///
///   **Note:** If your logs don’t contain any of the default attributes
///   and you haven’t defined your own date attribute, Datadog timestamps
///   the logs with the date it received them.
///
///   If multiple log date remapper processors can be applied to a given log,
///   only the first one (according to the pipelines order) is taken into account.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LogsDateRemapper {
    /// Whether or not the processor is enabled.
    #[serde(rename = "is_enabled")]
    pub is_enabled: Option<bool>,
    /// Name of the processor.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Array of source attributes.
    #[serde(rename = "sources")]
    pub sources: Vec<String>,
    /// Type of logs date remapper.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::LogsDateRemapperType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LogsDateRemapper {
    pub fn new(
        sources: Vec<String>,
        type_: crate::datadogV1::model::LogsDateRemapperType,
    ) -> LogsDateRemapper {
        LogsDateRemapper {
            is_enabled: None,
            name: None,
            sources,
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
}

impl<'de> Deserialize<'de> for LogsDateRemapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogsDateRemapperVisitor;
        impl<'a> Visitor<'a> for LogsDateRemapperVisitor {
            type Value = LogsDateRemapper;

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
                let mut type_: Option<crate::datadogV1::model::LogsDateRemapperType> = None;
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
                            sources = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::LogsDateRemapperType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }
                let sources = sources.ok_or_else(|| M::Error::missing_field("sources"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = LogsDateRemapper {
                    is_enabled,
                    name,
                    sources,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogsDateRemapperVisitor)
    }
}

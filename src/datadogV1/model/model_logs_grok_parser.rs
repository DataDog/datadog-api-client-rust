// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Create custom grok rules to parse the full message or [a specific attribute of your raw event](<https://docs.datadoghq.com/logs/log_configuration/parsing/#advanced-settings>).
/// For more information, see the [parsing section](<https://docs.datadoghq.com/logs/log_configuration/parsing>).
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LogsGrokParser {
    /// Set of rules for the grok parser.
    #[serde(rename = "grok")]
    pub grok: crate::datadogV1::model::LogsGrokParserRules,
    /// Whether or not the processor is enabled.
    #[serde(rename = "is_enabled")]
    pub is_enabled: Option<bool>,
    /// Name of the processor.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// List of sample logs to test this grok parser.
    #[serde(rename = "samples")]
    pub samples: Option<Vec<String>>,
    /// Name of the log attribute to parse.
    #[serde(rename = "source")]
    pub source: String,
    /// Type of logs grok parser.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::LogsGrokParserType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LogsGrokParser {
    pub fn new(
        grok: crate::datadogV1::model::LogsGrokParserRules,
        source: String,
        type_: crate::datadogV1::model::LogsGrokParserType,
    ) -> LogsGrokParser {
        LogsGrokParser {
            grok,
            is_enabled: None,
            name: None,
            samples: None,
            source,
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

    pub fn samples(mut self, value: Vec<String>) -> Self {
        self.samples = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for LogsGrokParser {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogsGrokParserVisitor;
        impl<'a> Visitor<'a> for LogsGrokParserVisitor {
            type Value = LogsGrokParser;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut grok: Option<crate::datadogV1::model::LogsGrokParserRules> = None;
                let mut is_enabled: Option<bool> = None;
                let mut name: Option<String> = None;
                let mut samples: Option<Vec<String>> = None;
                let mut source: Option<String> = None;
                let mut type_: Option<crate::datadogV1::model::LogsGrokParserType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "grok" => {
                            grok = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
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
                        "samples" => {
                            if v.is_null() {
                                continue;
                            }
                            samples = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source" => {
                            source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::LogsGrokParserType::UnparsedObject(
                                        _type_,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }
                let grok = grok.ok_or_else(|| M::Error::missing_field("grok"))?;
                let source = source.ok_or_else(|| M::Error::missing_field("source"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = LogsGrokParser {
                    grok,
                    is_enabled,
                    name,
                    samples,
                    source,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogsGrokParserVisitor)
    }
}

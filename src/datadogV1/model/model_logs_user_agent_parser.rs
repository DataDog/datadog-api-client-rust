// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The User-Agent parser takes a User-Agent attribute and extracts the OS, browser, device, and other user data.
/// It recognizes major bots like the Google Bot, Yahoo Slurp, and Bing.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LogsUserAgentParser {
    /// Whether or not the processor is enabled.
    #[serde(rename = "is_enabled")]
    pub is_enabled: Option<bool>,
    /// Define if the source attribute is URL encoded or not.
    #[serde(rename = "is_encoded")]
    pub is_encoded: Option<bool>,
    /// Name of the processor.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Array of source attributes.
    #[serde(rename = "sources")]
    pub sources: Vec<String>,
    /// Name of the parent attribute that contains all the extracted details from the `sources`.
    #[serde(rename = "target")]
    pub target: String,
    /// Type of logs User-Agent parser.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::LogsUserAgentParserType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LogsUserAgentParser {
    pub fn new(
        sources: Vec<String>,
        target: String,
        type_: crate::datadogV1::model::LogsUserAgentParserType,
    ) -> LogsUserAgentParser {
        LogsUserAgentParser {
            is_enabled: None,
            is_encoded: None,
            name: None,
            sources,
            target,
            type_,
            _unparsed: false,
        }
    }

    pub fn is_enabled(mut self, value: bool) -> Self {
        self.is_enabled = Some(value);
        self
    }

    pub fn is_encoded(mut self, value: bool) -> Self {
        self.is_encoded = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for LogsUserAgentParser {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogsUserAgentParserVisitor;
        impl<'a> Visitor<'a> for LogsUserAgentParserVisitor {
            type Value = LogsUserAgentParser;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut is_enabled: Option<bool> = None;
                let mut is_encoded: Option<bool> = None;
                let mut name: Option<String> = None;
                let mut sources: Option<Vec<String>> = None;
                let mut target: Option<String> = None;
                let mut type_: Option<crate::datadogV1::model::LogsUserAgentParserType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "is_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            is_enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_encoded" => {
                            if v.is_null() {
                                continue;
                            }
                            is_encoded = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "target" => {
                            target = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::LogsUserAgentParserType::UnparsedObject(_type_) => {
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
                let target = target.ok_or_else(|| M::Error::missing_field("target"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = LogsUserAgentParser {
                    is_enabled,
                    is_encoded,
                    name,
                    sources,
                    target,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogsUserAgentParserVisitor)
    }
}

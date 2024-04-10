// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// JSON object containing all log attributes and their associated values.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LogContent {
    /// JSON object of attributes from your log.
    #[serde(rename = "attributes")]
    pub attributes: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// Name of the machine from where the logs are being sent.
    #[serde(rename = "host")]
    pub host: Option<String>,
    /// The message [reserved attribute](<https://docs.datadoghq.com/logs/log_collection/#reserved-attributes>)
    /// of your log. By default, Datadog ingests the value of the message attribute as the body of the log entry.
    /// That value is then highlighted and displayed in the Logstream, where it is indexed for full text search.
    #[serde(rename = "message")]
    pub message: Option<String>,
    /// The name of the application or service generating the log events.
    /// It is used to switch from Logs to APM, so make sure you define the same
    /// value when you use both products.
    #[serde(rename = "service")]
    pub service: Option<String>,
    /// Array of tags associated with your log.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// Timestamp of your log.
    #[serde(rename = "timestamp")]
    pub timestamp: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LogContent {
    pub fn new() -> LogContent {
        LogContent {
            attributes: None,
            host: None,
            message: None,
            service: None,
            tags: None,
            timestamp: None,
            _unparsed: false,
        }
    }

    pub fn attributes(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.attributes = Some(value);
        self
    }

    pub fn host(mut self, value: String) -> Self {
        self.host = Some(value);
        self
    }

    pub fn message(mut self, value: String) -> Self {
        self.message = Some(value);
        self
    }

    pub fn service(mut self, value: String) -> Self {
        self.service = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn timestamp(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.timestamp = Some(value);
        self
    }
}

impl Default for LogContent {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for LogContent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogContentVisitor;
        impl<'a> Visitor<'a> for LogContentVisitor {
            type Value = LogContent;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attributes: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut host: Option<String> = None;
                let mut message: Option<String> = None;
                let mut service: Option<String> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut timestamp: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "attributes" => {
                            if v.is_null() {
                                continue;
                            }
                            attributes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "host" => {
                            if v.is_null() {
                                continue;
                            }
                            host = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "message" => {
                            if v.is_null() {
                                continue;
                            }
                            message = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "service" => {
                            if v.is_null() {
                                continue;
                            }
                            service = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timestamp" => {
                            if v.is_null() {
                                continue;
                            }
                            timestamp = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = LogContent {
                    attributes,
                    host,
                    message,
                    service,
                    tags,
                    timestamp,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogContentVisitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for a process summary.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProcessSummaryAttributes {
    /// Process command line.
    #[serde(rename = "cmdline")]
    pub cmdline: Option<String>,
    /// Host running the process.
    #[serde(rename = "host")]
    pub host: Option<String>,
    /// Process ID.
    #[serde(rename = "pid")]
    pub pid: Option<i64>,
    /// Parent process ID.
    #[serde(rename = "ppid")]
    pub ppid: Option<i64>,
    /// Time the process was started.
    #[serde(rename = "start")]
    pub start: Option<String>,
    /// List of tags associated with the process.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// Time the process was seen.
    #[serde(rename = "timestamp")]
    pub timestamp: Option<String>,
    /// Process owner.
    #[serde(rename = "user")]
    pub user: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProcessSummaryAttributes {
    pub fn new() -> ProcessSummaryAttributes {
        ProcessSummaryAttributes {
            cmdline: None,
            host: None,
            pid: None,
            ppid: None,
            start: None,
            tags: None,
            timestamp: None,
            user: None,
            _unparsed: false,
        }
    }

    pub fn cmdline(mut self, value: String) -> Self {
        self.cmdline = Some(value);
        self
    }

    pub fn host(mut self, value: String) -> Self {
        self.host = Some(value);
        self
    }

    pub fn pid(mut self, value: i64) -> Self {
        self.pid = Some(value);
        self
    }

    pub fn ppid(mut self, value: i64) -> Self {
        self.ppid = Some(value);
        self
    }

    pub fn start(mut self, value: String) -> Self {
        self.start = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn timestamp(mut self, value: String) -> Self {
        self.timestamp = Some(value);
        self
    }

    pub fn user(mut self, value: String) -> Self {
        self.user = Some(value);
        self
    }
}

impl Default for ProcessSummaryAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ProcessSummaryAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProcessSummaryAttributesVisitor;
        impl<'a> Visitor<'a> for ProcessSummaryAttributesVisitor {
            type Value = ProcessSummaryAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut cmdline: Option<String> = None;
                let mut host: Option<String> = None;
                let mut pid: Option<i64> = None;
                let mut ppid: Option<i64> = None;
                let mut start: Option<String> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut timestamp: Option<String> = None;
                let mut user: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "cmdline" => {
                            if v.is_null() {
                                continue;
                            }
                            cmdline = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "host" => {
                            if v.is_null() {
                                continue;
                            }
                            host = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "pid" => {
                            if v.is_null() {
                                continue;
                            }
                            pid = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ppid" => {
                            if v.is_null() {
                                continue;
                            }
                            ppid = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "start" => {
                            if v.is_null() {
                                continue;
                            }
                            start = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "user" => {
                            if v.is_null() {
                                continue;
                            }
                            user = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = ProcessSummaryAttributes {
                    cmdline,
                    host,
                    pid,
                    ppid,
                    start,
                    tags,
                    timestamp,
                    user,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProcessSummaryAttributesVisitor)
    }
}

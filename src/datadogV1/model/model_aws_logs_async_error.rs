// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Description of errors.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AWSLogsAsyncError {
    /// Code properties
    #[serde(rename = "code")]
    pub code: Option<String>,
    /// Message content.
    #[serde(rename = "message")]
    pub message: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AWSLogsAsyncError {
    pub fn new() -> AWSLogsAsyncError {
        AWSLogsAsyncError {
            code: None,
            message: None,
            _unparsed: false,
        }
    }

    pub fn code(mut self, value: String) -> Self {
        self.code = Some(value);
        self
    }

    pub fn message(mut self, value: String) -> Self {
        self.message = Some(value);
        self
    }
}

impl Default for AWSLogsAsyncError {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AWSLogsAsyncError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AWSLogsAsyncErrorVisitor;
        impl<'a> Visitor<'a> for AWSLogsAsyncErrorVisitor {
            type Value = AWSLogsAsyncError;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut code: Option<String> = None;
                let mut message: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "code" => {
                            if v.is_null() {
                                continue;
                            }
                            code = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "message" => {
                            if v.is_null() {
                                continue;
                            }
                            message = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = AWSLogsAsyncError {
                    code,
                    message,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AWSLogsAsyncErrorVisitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Response returned by the Logs API when errors occur.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LogsAPIErrorResponse {
    /// Error returned by the Logs API
    #[serde(rename = "error")]
    pub error: Option<crate::datadogV1::model::LogsAPIError>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LogsAPIErrorResponse {
    pub fn new() -> LogsAPIErrorResponse {
        LogsAPIErrorResponse {
            error: None,
            _unparsed: false,
        }
    }

    pub fn error(mut self, value: crate::datadogV1::model::LogsAPIError) -> Self {
        self.error = Some(value);
        self
    }
}

impl Default for LogsAPIErrorResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for LogsAPIErrorResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogsAPIErrorResponseVisitor;
        impl<'a> Visitor<'a> for LogsAPIErrorResponseVisitor {
            type Value = LogsAPIErrorResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut error: Option<crate::datadogV1::model::LogsAPIError> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "error" => {
                            if v.is_null() {
                                continue;
                            }
                            error = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = LogsAPIErrorResponse { error, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogsAPIErrorResponseVisitor)
    }
}

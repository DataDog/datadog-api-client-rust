// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Response object with all logs matching the request and pagination information.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LogsListResponse {
    /// Array of logs matching the request and the `nextLogId` if sent.
    #[serde(rename = "logs")]
    pub logs: Option<Vec<crate::datadogV1::model::Log>>,
    /// Hash identifier of the next log to return in the list.
    /// This parameter is used for the pagination feature.
    #[serde(
        rename = "nextLogId",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub next_log_id: Option<Option<String>>,
    /// Status of the response.
    #[serde(rename = "status")]
    pub status: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LogsListResponse {
    pub fn new() -> LogsListResponse {
        LogsListResponse {
            logs: None,
            next_log_id: None,
            status: None,
            _unparsed: false,
        }
    }

    pub fn logs(mut self, value: Vec<crate::datadogV1::model::Log>) -> Self {
        self.logs = Some(value);
        self
    }

    pub fn next_log_id(mut self, value: Option<String>) -> Self {
        self.next_log_id = Some(value);
        self
    }

    pub fn status(mut self, value: String) -> Self {
        self.status = Some(value);
        self
    }
}

impl Default for LogsListResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for LogsListResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogsListResponseVisitor;
        impl<'a> Visitor<'a> for LogsListResponseVisitor {
            type Value = LogsListResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut logs: Option<Vec<crate::datadogV1::model::Log>> = None;
                let mut next_log_id: Option<Option<String>> = None;
                let mut status: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "logs" => {
                            if v.is_null() {
                                continue;
                            }
                            logs = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "nextLogId" => {
                            next_log_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = LogsListResponse {
                    logs,
                    next_log_id,
                    status,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogsListResponseVisitor)
    }
}

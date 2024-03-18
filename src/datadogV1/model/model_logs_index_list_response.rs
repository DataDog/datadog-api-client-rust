// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object with all Index configurations for a given organization.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LogsIndexListResponse {
    /// Array of Log index configurations.
    #[serde(rename = "indexes")]
    pub indexes: Option<Vec<crate::datadogV1::model::LogsIndex>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LogsIndexListResponse {
    pub fn new() -> LogsIndexListResponse {
        LogsIndexListResponse {
            indexes: None,
            _unparsed: false,
        }
    }

    pub fn indexes(mut self, value: Vec<crate::datadogV1::model::LogsIndex>) -> Self {
        self.indexes = Some(value);
        self
    }
}

impl Default for LogsIndexListResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for LogsIndexListResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogsIndexListResponseVisitor;
        impl<'a> Visitor<'a> for LogsIndexListResponseVisitor {
            type Value = LogsIndexListResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut indexes: Option<Vec<crate::datadogV1::model::LogsIndex>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "indexes" => {
                            if v.is_null() {
                                continue;
                            }
                            indexes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = LogsIndexListResponse { indexes, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogsIndexListResponseVisitor)
    }
}

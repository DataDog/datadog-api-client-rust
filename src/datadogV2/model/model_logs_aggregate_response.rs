// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The response object for the logs aggregate API endpoint
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LogsAggregateResponse {
    /// The query results
    #[serde(rename = "data")]
    pub data: Option<crate::datadogV2::model::LogsAggregateResponseData>,
    /// The metadata associated with a request
    #[serde(rename = "meta")]
    pub meta: Option<crate::datadogV2::model::LogsResponseMetadata>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LogsAggregateResponse {
    pub fn new() -> LogsAggregateResponse {
        LogsAggregateResponse {
            data: None,
            meta: None,
            _unparsed: false,
        }
    }

    pub fn data(mut self, value: crate::datadogV2::model::LogsAggregateResponseData) -> Self {
        self.data = Some(value);
        self
    }

    pub fn meta(mut self, value: crate::datadogV2::model::LogsResponseMetadata) -> Self {
        self.meta = Some(value);
        self
    }
}

impl Default for LogsAggregateResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for LogsAggregateResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogsAggregateResponseVisitor;
        impl<'a> Visitor<'a> for LogsAggregateResponseVisitor {
            type Value = LogsAggregateResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data: Option<crate::datadogV2::model::LogsAggregateResponseData> = None;
                let mut meta: Option<crate::datadogV2::model::LogsResponseMetadata> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data" => {
                            if v.is_null() {
                                continue;
                            }
                            data = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "meta" => {
                            if v.is_null() {
                                continue;
                            }
                            meta = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = LogsAggregateResponse {
                    data,
                    meta,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogsAggregateResponseVisitor)
    }
}

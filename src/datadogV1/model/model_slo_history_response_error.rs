// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A list of errors while querying the history data for the service level objective.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SLOHistoryResponseError {
    /// Human readable error.
    #[serde(rename = "error")]
    pub error: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SLOHistoryResponseError {
    pub fn new() -> SLOHistoryResponseError {
        SLOHistoryResponseError {
            error: None,
            _unparsed: false,
        }
    }

    pub fn error(&mut self, value: String) -> &mut Self {
        self.error = Some(value);
        self
    }
}

impl Default for SLOHistoryResponseError {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SLOHistoryResponseError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SLOHistoryResponseErrorVisitor;
        impl<'a> Visitor<'a> for SLOHistoryResponseErrorVisitor {
            type Value = SLOHistoryResponseError;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut error: Option<String> = None;
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

                let content = SLOHistoryResponseError { error, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SLOHistoryResponseErrorVisitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object describing a log after being processed and stored by Datadog.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Log {
    /// JSON object containing all log attributes and their associated values.
    #[serde(rename = "content")]
    pub content: Option<crate::datadogV1::model::LogContent>,
    /// Unique ID of the Log.
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl Log {
    pub fn new() -> Log {
        Log {
            content: None,
            id: None,
            _unparsed: false,
        }
    }

    pub fn content(mut self, value: crate::datadogV1::model::LogContent) -> Self {
        self.content = Some(value);
        self
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }
}

impl Default for Log {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for Log {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogVisitor;
        impl<'a> Visitor<'a> for LogVisitor {
            type Value = Log;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut content: Option<crate::datadogV1::model::LogContent> = None;
                let mut id: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "content" => {
                            if v.is_null() {
                                continue;
                            }
                            content = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = Log {
                    content,
                    id,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogVisitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Invalid query performed.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct HTTPLogError {
    /// Error code.
    #[serde(rename = "code")]
    pub code: i32,
    /// Error message.
    #[serde(rename = "message")]
    pub message: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl HTTPLogError {
    pub fn new(code: i32, message: String) -> HTTPLogError {
        HTTPLogError {
            code,
            message,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for HTTPLogError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HTTPLogErrorVisitor;
        impl<'a> Visitor<'a> for HTTPLogErrorVisitor {
            type Value = HTTPLogError;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut code: Option<i32> = None;
                let mut message: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "code" => {
                            code = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "message" => {
                            message = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let code = code.ok_or_else(|| M::Error::missing_field("code"))?;
                let message = message.ok_or_else(|| M::Error::missing_field("message"))?;

                let content = HTTPLogError {
                    code,
                    message,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(HTTPLogErrorVisitor)
    }
}

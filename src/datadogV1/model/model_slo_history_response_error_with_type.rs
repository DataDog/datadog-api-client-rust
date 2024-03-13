// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// An object describing the error with error type and error message.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SLOHistoryResponseErrorWithType {
    /// A message with more details about the error.
    #[serde(rename = "error_message")]
    pub error_message: String,
    /// Type of the error.
    #[serde(rename = "error_type")]
    pub error_type: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SLOHistoryResponseErrorWithType {
    pub fn new(error_message: String, error_type: String) -> SLOHistoryResponseErrorWithType {
        SLOHistoryResponseErrorWithType {
            error_message,
            error_type,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for SLOHistoryResponseErrorWithType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SLOHistoryResponseErrorWithTypeVisitor;
        impl<'a> Visitor<'a> for SLOHistoryResponseErrorWithTypeVisitor {
            type Value = SLOHistoryResponseErrorWithType;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut error_message: Option<String> = None;
                let mut error_type: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "error_message" => {
                            error_message =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "error_type" => {
                            error_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let error_message =
                    error_message.ok_or_else(|| M::Error::missing_field("error_message"))?;
                let error_type = error_type.ok_or_else(|| M::Error::missing_field("error_type"))?;

                let content = SLOHistoryResponseErrorWithType {
                    error_message,
                    error_type,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SLOHistoryResponseErrorWithTypeVisitor)
    }
}

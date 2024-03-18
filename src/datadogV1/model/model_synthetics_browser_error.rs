// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Error response object for a browser test.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsBrowserError {
    /// Description of the error.
    #[serde(rename = "description")]
    pub description: String,
    /// Name of the error.
    #[serde(rename = "name")]
    pub name: String,
    /// Status Code of the error.
    #[serde(rename = "status")]
    pub status: Option<i64>,
    /// Error type returned by a browser test.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::SyntheticsBrowserErrorType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsBrowserError {
    pub fn new(
        description: String,
        name: String,
        type_: crate::datadogV1::model::SyntheticsBrowserErrorType,
    ) -> SyntheticsBrowserError {
        SyntheticsBrowserError {
            description,
            name,
            status: None,
            type_,
            _unparsed: false,
        }
    }

    pub fn status(mut self, value: i64) -> Self {
        self.status = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for SyntheticsBrowserError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsBrowserErrorVisitor;
        impl<'a> Visitor<'a> for SyntheticsBrowserErrorVisitor {
            type Value = SyntheticsBrowserError;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut description: Option<String> = None;
                let mut name: Option<String> = None;
                let mut status: Option<i64> = None;
                let mut type_: Option<crate::datadogV1::model::SyntheticsBrowserErrorType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::SyntheticsBrowserErrorType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = SyntheticsBrowserError {
                    description,
                    name,
                    status,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsBrowserErrorVisitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Error response object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct APIErrorResponse {
    /// Array of errors returned by the API.
    #[serde(rename = "errors")]
    pub errors: Vec<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl APIErrorResponse {
    pub fn new(errors: Vec<String>) -> APIErrorResponse {
        APIErrorResponse {
            errors,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for APIErrorResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct APIErrorResponseVisitor;
        impl<'a> Visitor<'a> for APIErrorResponseVisitor {
            type Value = APIErrorResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut errors: Option<Vec<String>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "errors" => {
                            errors = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let errors = errors.ok_or_else(|| M::Error::missing_field("errors"))?;

                let content = APIErrorResponse { errors, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(APIErrorResponseVisitor)
    }
}

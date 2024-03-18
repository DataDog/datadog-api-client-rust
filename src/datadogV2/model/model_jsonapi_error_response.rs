// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// API error response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct JSONAPIErrorResponse {
    /// A list of errors.
    #[serde(rename = "errors")]
    pub errors: Vec<crate::datadogV2::model::JSONAPIErrorItem>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl JSONAPIErrorResponse {
    pub fn new(errors: Vec<crate::datadogV2::model::JSONAPIErrorItem>) -> JSONAPIErrorResponse {
        JSONAPIErrorResponse {
            errors,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for JSONAPIErrorResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct JSONAPIErrorResponseVisitor;
        impl<'a> Visitor<'a> for JSONAPIErrorResponseVisitor {
            type Value = JSONAPIErrorResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut errors: Option<Vec<crate::datadogV2::model::JSONAPIErrorItem>> = None;
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

                let content = JSONAPIErrorResponse { errors, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(JSONAPIErrorResponseVisitor)
    }
}

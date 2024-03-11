// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Errors occurred.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct HTTPCIAppErrors {
    /// Structured errors.
    #[serde(rename = "errors")]
    pub errors: Option<Vec<crate::datadogV2::model::HTTPCIAppError>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl HTTPCIAppErrors {
    pub fn new() -> HTTPCIAppErrors {
        HTTPCIAppErrors {
            errors: None,
            _unparsed: false,
        }
    }

    pub fn errors(&mut self, value: Vec<crate::datadogV2::model::HTTPCIAppError>) -> &mut Self {
        self.errors = Some(value);
        self
    }
}

impl Default for HTTPCIAppErrors {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for HTTPCIAppErrors {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HTTPCIAppErrorsVisitor;
        impl<'a> Visitor<'a> for HTTPCIAppErrorsVisitor {
            type Value = HTTPCIAppErrors;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut errors: Option<Vec<crate::datadogV2::model::HTTPCIAppError>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "errors" => {
                            if v.is_null() {
                                continue;
                            }
                            errors = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = HTTPCIAppErrors { errors, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(HTTPCIAppErrorsVisitor)
    }
}

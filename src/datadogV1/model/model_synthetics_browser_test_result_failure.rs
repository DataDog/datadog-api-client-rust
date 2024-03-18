// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The browser test failure details.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsBrowserTestResultFailure {
    /// Error code that can be returned by a Synthetic test.
    #[serde(rename = "code")]
    pub code: Option<crate::datadogV1::model::SyntheticsBrowserTestFailureCode>,
    /// The browser test error message.
    #[serde(rename = "message")]
    pub message: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsBrowserTestResultFailure {
    pub fn new() -> SyntheticsBrowserTestResultFailure {
        SyntheticsBrowserTestResultFailure {
            code: None,
            message: None,
            _unparsed: false,
        }
    }

    pub fn code(
        mut self,
        value: crate::datadogV1::model::SyntheticsBrowserTestFailureCode,
    ) -> Self {
        self.code = Some(value);
        self
    }

    pub fn message(mut self, value: String) -> Self {
        self.message = Some(value);
        self
    }
}

impl Default for SyntheticsBrowserTestResultFailure {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsBrowserTestResultFailure {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsBrowserTestResultFailureVisitor;
        impl<'a> Visitor<'a> for SyntheticsBrowserTestResultFailureVisitor {
            type Value = SyntheticsBrowserTestResultFailure;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut code: Option<crate::datadogV1::model::SyntheticsBrowserTestFailureCode> =
                    None;
                let mut message: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "code" => {
                            if v.is_null() {
                                continue;
                            }
                            code = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _code) = code {
                                match _code {
                                    crate::datadogV1::model::SyntheticsBrowserTestFailureCode::UnparsedObject(_code) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "message" => {
                            if v.is_null() {
                                continue;
                            }
                            message = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SyntheticsBrowserTestResultFailure {
                    code,
                    message,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsBrowserTestResultFailureVisitor)
    }
}

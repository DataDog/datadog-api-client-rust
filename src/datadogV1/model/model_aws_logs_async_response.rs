// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A list of all Datadog-AWS logs integrations available in your Datadog organization.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AWSLogsAsyncResponse {
    /// List of errors.
    #[serde(rename = "errors")]
    pub errors: Option<Vec<crate::datadogV1::model::AWSLogsAsyncError>>,
    /// Status of the properties.
    #[serde(rename = "status")]
    pub status: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AWSLogsAsyncResponse {
    pub fn new() -> AWSLogsAsyncResponse {
        AWSLogsAsyncResponse {
            errors: None,
            status: None,
            _unparsed: false,
        }
    }

    pub fn errors(mut self, value: Vec<crate::datadogV1::model::AWSLogsAsyncError>) -> Self {
        self.errors = Some(value);
        self
    }

    pub fn status(mut self, value: String) -> Self {
        self.status = Some(value);
        self
    }
}

impl Default for AWSLogsAsyncResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AWSLogsAsyncResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AWSLogsAsyncResponseVisitor;
        impl<'a> Visitor<'a> for AWSLogsAsyncResponseVisitor {
            type Value = AWSLogsAsyncResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut errors: Option<Vec<crate::datadogV1::model::AWSLogsAsyncError>> = None;
                let mut status: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "errors" => {
                            if v.is_null() {
                                continue;
                            }
                            errors = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = AWSLogsAsyncResponse {
                    errors,
                    status,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AWSLogsAsyncResponseVisitor)
    }
}

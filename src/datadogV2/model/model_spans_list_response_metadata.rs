// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The metadata associated with a request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SpansListResponseMetadata {
    /// The time elapsed in milliseconds.
    #[serde(rename = "elapsed")]
    pub elapsed: Option<i64>,
    /// Paging attributes.
    #[serde(rename = "page")]
    pub page: Option<crate::datadogV2::model::SpansResponseMetadataPage>,
    /// The identifier of the request.
    #[serde(rename = "request_id")]
    pub request_id: Option<String>,
    /// The status of the response.
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV2::model::SpansAggregateResponseStatus>,
    /// A list of warnings (non fatal errors) encountered, partial results might be returned if
    /// warnings are present in the response.
    #[serde(rename = "warnings")]
    pub warnings: Option<Vec<crate::datadogV2::model::SpansWarning>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SpansListResponseMetadata {
    pub fn new() -> SpansListResponseMetadata {
        SpansListResponseMetadata {
            elapsed: None,
            page: None,
            request_id: None,
            status: None,
            warnings: None,
            _unparsed: false,
        }
    }

    pub fn elapsed(mut self, value: i64) -> Self {
        self.elapsed = Some(value);
        self
    }

    pub fn page(mut self, value: crate::datadogV2::model::SpansResponseMetadataPage) -> Self {
        self.page = Some(value);
        self
    }

    pub fn request_id(mut self, value: String) -> Self {
        self.request_id = Some(value);
        self
    }

    pub fn status(mut self, value: crate::datadogV2::model::SpansAggregateResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn warnings(mut self, value: Vec<crate::datadogV2::model::SpansWarning>) -> Self {
        self.warnings = Some(value);
        self
    }
}

impl Default for SpansListResponseMetadata {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SpansListResponseMetadata {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SpansListResponseMetadataVisitor;
        impl<'a> Visitor<'a> for SpansListResponseMetadataVisitor {
            type Value = SpansListResponseMetadata;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut elapsed: Option<i64> = None;
                let mut page: Option<crate::datadogV2::model::SpansResponseMetadataPage> = None;
                let mut request_id: Option<String> = None;
                let mut status: Option<crate::datadogV2::model::SpansAggregateResponseStatus> =
                    None;
                let mut warnings: Option<Vec<crate::datadogV2::model::SpansWarning>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "elapsed" => {
                            if v.is_null() {
                                continue;
                            }
                            elapsed = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "page" => {
                            if v.is_null() {
                                continue;
                            }
                            page = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "request_id" => {
                            if v.is_null() {
                                continue;
                            }
                            request_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::SpansAggregateResponseStatus::UnparsedObject(_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "warnings" => {
                            if v.is_null() {
                                continue;
                            }
                            warnings = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SpansListResponseMetadata {
                    elapsed,
                    page,
                    request_id,
                    status,
                    warnings,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SpansListResponseMetadataVisitor)
    }
}

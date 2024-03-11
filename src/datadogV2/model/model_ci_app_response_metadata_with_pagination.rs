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
pub struct CIAppResponseMetadataWithPagination {
    /// The time elapsed in milliseconds.
    #[serde(rename = "elapsed")]
    pub elapsed: Option<i64>,
    /// Paging attributes.
    #[serde(rename = "page")]
    pub page: Option<crate::datadogV2::model::CIAppResponsePage>,
    /// The identifier of the request.
    #[serde(rename = "request_id")]
    pub request_id: Option<String>,
    /// The status of the response.
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV2::model::CIAppResponseStatus>,
    /// A list of warnings (non-fatal errors) encountered. Partial results may return if
    /// warnings are present in the response.
    #[serde(rename = "warnings")]
    pub warnings: Option<Vec<crate::datadogV2::model::CIAppWarning>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CIAppResponseMetadataWithPagination {
    pub fn new() -> CIAppResponseMetadataWithPagination {
        CIAppResponseMetadataWithPagination {
            elapsed: None,
            page: None,
            request_id: None,
            status: None,
            warnings: None,
            _unparsed: false,
        }
    }

    pub fn elapsed(&mut self, value: i64) -> &mut Self {
        self.elapsed = Some(value);
        self
    }

    pub fn page(&mut self, value: crate::datadogV2::model::CIAppResponsePage) -> &mut Self {
        self.page = Some(value);
        self
    }

    pub fn request_id(&mut self, value: String) -> &mut Self {
        self.request_id = Some(value);
        self
    }

    pub fn status(&mut self, value: crate::datadogV2::model::CIAppResponseStatus) -> &mut Self {
        self.status = Some(value);
        self
    }

    pub fn warnings(&mut self, value: Vec<crate::datadogV2::model::CIAppWarning>) -> &mut Self {
        self.warnings = Some(value);
        self
    }
}

impl Default for CIAppResponseMetadataWithPagination {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CIAppResponseMetadataWithPagination {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CIAppResponseMetadataWithPaginationVisitor;
        impl<'a> Visitor<'a> for CIAppResponseMetadataWithPaginationVisitor {
            type Value = CIAppResponseMetadataWithPagination;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut elapsed: Option<i64> = None;
                let mut page: Option<crate::datadogV2::model::CIAppResponsePage> = None;
                let mut request_id: Option<String> = None;
                let mut status: Option<crate::datadogV2::model::CIAppResponseStatus> = None;
                let mut warnings: Option<Vec<crate::datadogV2::model::CIAppWarning>> = None;
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
                                    crate::datadogV2::model::CIAppResponseStatus::UnparsedObject(_status) => {
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

                let content = CIAppResponseMetadataWithPagination {
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

        deserializer.deserialize_any(CIAppResponseMetadataWithPaginationVisitor)
    }
}

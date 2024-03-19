// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The bulk partial delete service level objective object endpoint
/// response.
///
/// This endpoint operates on multiple service level objective objects, so
/// it may be partially successful. In such cases, the "data" and "error"
/// fields in this response indicate which deletions succeeded and failed.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SLOBulkDeleteResponse {
    /// An array of service level objective objects.
    #[serde(rename = "data")]
    pub data: Option<crate::datadogV1::model::SLOBulkDeleteResponseData>,
    /// Array of errors object returned.
    #[serde(rename = "errors")]
    pub errors: Option<Vec<crate::datadogV1::model::SLOBulkDeleteError>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SLOBulkDeleteResponse {
    pub fn new() -> SLOBulkDeleteResponse {
        SLOBulkDeleteResponse {
            data: None,
            errors: None,
            _unparsed: false,
        }
    }

    pub fn data(mut self, value: crate::datadogV1::model::SLOBulkDeleteResponseData) -> Self {
        self.data = Some(value);
        self
    }

    pub fn errors(mut self, value: Vec<crate::datadogV1::model::SLOBulkDeleteError>) -> Self {
        self.errors = Some(value);
        self
    }
}

impl Default for SLOBulkDeleteResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SLOBulkDeleteResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SLOBulkDeleteResponseVisitor;
        impl<'a> Visitor<'a> for SLOBulkDeleteResponseVisitor {
            type Value = SLOBulkDeleteResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data: Option<crate::datadogV1::model::SLOBulkDeleteResponseData> = None;
                let mut errors: Option<Vec<crate::datadogV1::model::SLOBulkDeleteError>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data" => {
                            if v.is_null() {
                                continue;
                            }
                            data = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "errors" => {
                            if v.is_null() {
                                continue;
                            }
                            errors = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SLOBulkDeleteResponse {
                    data,
                    errors,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SLOBulkDeleteResponseVisitor)
    }
}

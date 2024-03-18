// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Create group request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SensitiveDataScannerGroupCreateRequest {
    /// Data related to the creation of a group.
    #[serde(rename = "data")]
    pub data: Option<crate::datadogV2::model::SensitiveDataScannerGroupCreate>,
    /// Meta payload containing information about the API.
    #[serde(rename = "meta")]
    pub meta: Option<crate::datadogV2::model::SensitiveDataScannerMetaVersionOnly>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SensitiveDataScannerGroupCreateRequest {
    pub fn new() -> SensitiveDataScannerGroupCreateRequest {
        SensitiveDataScannerGroupCreateRequest {
            data: None,
            meta: None,
            _unparsed: false,
        }
    }

    pub fn data(mut self, value: crate::datadogV2::model::SensitiveDataScannerGroupCreate) -> Self {
        self.data = Some(value);
        self
    }

    pub fn meta(
        mut self,
        value: crate::datadogV2::model::SensitiveDataScannerMetaVersionOnly,
    ) -> Self {
        self.meta = Some(value);
        self
    }
}

impl Default for SensitiveDataScannerGroupCreateRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SensitiveDataScannerGroupCreateRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SensitiveDataScannerGroupCreateRequestVisitor;
        impl<'a> Visitor<'a> for SensitiveDataScannerGroupCreateRequestVisitor {
            type Value = SensitiveDataScannerGroupCreateRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data: Option<crate::datadogV2::model::SensitiveDataScannerGroupCreate> =
                    None;
                let mut meta: Option<crate::datadogV2::model::SensitiveDataScannerMetaVersionOnly> =
                    None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data" => {
                            if v.is_null() {
                                continue;
                            }
                            data = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "meta" => {
                            if v.is_null() {
                                continue;
                            }
                            meta = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SensitiveDataScannerGroupCreateRequest {
                    data,
                    meta,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SensitiveDataScannerGroupCreateRequestVisitor)
    }
}

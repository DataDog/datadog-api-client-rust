// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Response for Uploaded Custom Costs files.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CustomCostsFileUploadResponse {
    /// JSON API format for a Custom Costs file.
    #[serde(rename = "data")]
    pub data: Option<crate::datadogV2::model::CustomCostsFileMetadataHighLevel>,
    /// Meta for the response from the Upload Custom Costs endpoints.
    #[serde(rename = "meta")]
    pub meta: Option<crate::datadogV2::model::CustomCostUploadResponseMeta>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CustomCostsFileUploadResponse {
    pub fn new() -> CustomCostsFileUploadResponse {
        CustomCostsFileUploadResponse {
            data: None,
            meta: None,
            _unparsed: false,
        }
    }

    pub fn data(
        mut self,
        value: crate::datadogV2::model::CustomCostsFileMetadataHighLevel,
    ) -> Self {
        self.data = Some(value);
        self
    }

    pub fn meta(mut self, value: crate::datadogV2::model::CustomCostUploadResponseMeta) -> Self {
        self.meta = Some(value);
        self
    }
}

impl Default for CustomCostsFileUploadResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CustomCostsFileUploadResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CustomCostsFileUploadResponseVisitor;
        impl<'a> Visitor<'a> for CustomCostsFileUploadResponseVisitor {
            type Value = CustomCostsFileUploadResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data: Option<crate::datadogV2::model::CustomCostsFileMetadataHighLevel> =
                    None;
                let mut meta: Option<crate::datadogV2::model::CustomCostUploadResponseMeta> = None;
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

                let content = CustomCostsFileUploadResponse {
                    data,
                    meta,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CustomCostsFileUploadResponseVisitor)
    }
}

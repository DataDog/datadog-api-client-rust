// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Presigned URL parameters returned for a multipart upload.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTestFileMultipartPresignedUrlsParams {
    /// The full storage path for the file being uploaded.
    #[serde(rename = "key")]
    pub key: Option<String>,
    /// The upload ID assigned by the storage provider for this multipart upload.
    #[serde(rename = "upload_id")]
    pub upload_id: Option<String>,
    /// A map of part numbers to presigned upload URLs.
    #[serde(rename = "urls")]
    pub urls: Option<std::collections::BTreeMap<String, String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTestFileMultipartPresignedUrlsParams {
    pub fn new() -> SyntheticsTestFileMultipartPresignedUrlsParams {
        SyntheticsTestFileMultipartPresignedUrlsParams {
            key: None,
            upload_id: None,
            urls: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn key(mut self, value: String) -> Self {
        self.key = Some(value);
        self
    }

    pub fn upload_id(mut self, value: String) -> Self {
        self.upload_id = Some(value);
        self
    }

    pub fn urls(mut self, value: std::collections::BTreeMap<String, String>) -> Self {
        self.urls = Some(value);
        self
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl Default for SyntheticsTestFileMultipartPresignedUrlsParams {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsTestFileMultipartPresignedUrlsParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTestFileMultipartPresignedUrlsParamsVisitor;
        impl<'a> Visitor<'a> for SyntheticsTestFileMultipartPresignedUrlsParamsVisitor {
            type Value = SyntheticsTestFileMultipartPresignedUrlsParams;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut key: Option<String> = None;
                let mut upload_id: Option<String> = None;
                let mut urls: Option<std::collections::BTreeMap<String, String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "key" => {
                            if v.is_null() {
                                continue;
                            }
                            key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "upload_id" => {
                            if v.is_null() {
                                continue;
                            }
                            upload_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "urls" => {
                            if v.is_null() {
                                continue;
                            }
                            urls = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsTestFileMultipartPresignedUrlsParams {
                    key,
                    upload_id,
                    urls,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTestFileMultipartPresignedUrlsParamsVisitor)
    }
}

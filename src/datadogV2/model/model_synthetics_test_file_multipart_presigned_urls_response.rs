// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Response containing presigned URLs for multipart file upload and the bucket key.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTestFileMultipartPresignedUrlsResponse {
    /// The bucket key that references the uploaded file after completion.
    #[serde(rename = "bucketKey")]
    pub bucket_key: Option<String>,
    /// Presigned URL parameters returned for a multipart upload.
    #[serde(rename = "multipart_presigned_urls_params")]
    pub multipart_presigned_urls_params:
        Option<crate::datadogV2::model::SyntheticsTestFileMultipartPresignedUrlsParams>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTestFileMultipartPresignedUrlsResponse {
    pub fn new() -> SyntheticsTestFileMultipartPresignedUrlsResponse {
        SyntheticsTestFileMultipartPresignedUrlsResponse {
            bucket_key: None,
            multipart_presigned_urls_params: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn bucket_key(mut self, value: String) -> Self {
        self.bucket_key = Some(value);
        self
    }

    pub fn multipart_presigned_urls_params(
        mut self,
        value: crate::datadogV2::model::SyntheticsTestFileMultipartPresignedUrlsParams,
    ) -> Self {
        self.multipart_presigned_urls_params = Some(value);
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

impl Default for SyntheticsTestFileMultipartPresignedUrlsResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsTestFileMultipartPresignedUrlsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTestFileMultipartPresignedUrlsResponseVisitor;
        impl<'a> Visitor<'a> for SyntheticsTestFileMultipartPresignedUrlsResponseVisitor {
            type Value = SyntheticsTestFileMultipartPresignedUrlsResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut bucket_key: Option<String> = None;
                let mut multipart_presigned_urls_params: Option<
                    crate::datadogV2::model::SyntheticsTestFileMultipartPresignedUrlsParams,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "bucketKey" => {
                            if v.is_null() {
                                continue;
                            }
                            bucket_key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "multipart_presigned_urls_params" => {
                            if v.is_null() {
                                continue;
                            }
                            multipart_presigned_urls_params =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsTestFileMultipartPresignedUrlsResponse {
                    bucket_key,
                    multipart_presigned_urls_params,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTestFileMultipartPresignedUrlsResponseVisitor)
    }
}

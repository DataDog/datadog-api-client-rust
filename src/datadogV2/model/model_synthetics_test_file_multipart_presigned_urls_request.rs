// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Request body for getting presigned URLs for a multipart file upload.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTestFileMultipartPresignedUrlsRequest {
    /// The bucket key prefix indicating the type of file upload.
    #[serde(rename = "bucketKeyPrefix")]
    pub bucket_key_prefix:
        crate::datadogV2::model::SyntheticsTestFileMultipartPresignedUrlsRequestBucketKeyPrefix,
    /// Array of part descriptors for the multipart upload.
    #[serde(rename = "parts")]
    pub parts: Vec<crate::datadogV2::model::SyntheticsTestFileMultipartPresignedUrlsPart>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTestFileMultipartPresignedUrlsRequest {
    pub fn new(
        bucket_key_prefix: crate::datadogV2::model::SyntheticsTestFileMultipartPresignedUrlsRequestBucketKeyPrefix,
        parts: Vec<crate::datadogV2::model::SyntheticsTestFileMultipartPresignedUrlsPart>,
    ) -> SyntheticsTestFileMultipartPresignedUrlsRequest {
        SyntheticsTestFileMultipartPresignedUrlsRequest {
            bucket_key_prefix,
            parts,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for SyntheticsTestFileMultipartPresignedUrlsRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTestFileMultipartPresignedUrlsRequestVisitor;
        impl<'a> Visitor<'a> for SyntheticsTestFileMultipartPresignedUrlsRequestVisitor {
            type Value = SyntheticsTestFileMultipartPresignedUrlsRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut bucket_key_prefix: Option<crate::datadogV2::model::SyntheticsTestFileMultipartPresignedUrlsRequestBucketKeyPrefix> = None;
                let mut parts: Option<
                    Vec<crate::datadogV2::model::SyntheticsTestFileMultipartPresignedUrlsPart>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "bucketKeyPrefix" => {
                            bucket_key_prefix =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _bucket_key_prefix) = bucket_key_prefix {
                                match _bucket_key_prefix {
                                    crate::datadogV2::model::SyntheticsTestFileMultipartPresignedUrlsRequestBucketKeyPrefix::UnparsedObject(_bucket_key_prefix) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "parts" => {
                            parts = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let bucket_key_prefix = bucket_key_prefix
                    .ok_or_else(|| M::Error::missing_field("bucket_key_prefix"))?;
                let parts = parts.ok_or_else(|| M::Error::missing_field("parts"))?;

                let content = SyntheticsTestFileMultipartPresignedUrlsRequest {
                    bucket_key_prefix,
                    parts,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTestFileMultipartPresignedUrlsRequestVisitor)
    }
}

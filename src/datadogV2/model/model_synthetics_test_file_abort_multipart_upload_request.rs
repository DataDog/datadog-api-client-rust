// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Request body for aborting a multipart file upload.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTestFileAbortMultipartUploadRequest {
    /// The full storage path of the file whose upload should be aborted.
    #[serde(rename = "key")]
    pub key: String,
    /// The upload ID of the multipart upload to abort.
    #[serde(rename = "uploadId")]
    pub upload_id: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTestFileAbortMultipartUploadRequest {
    pub fn new(key: String, upload_id: String) -> SyntheticsTestFileAbortMultipartUploadRequest {
        SyntheticsTestFileAbortMultipartUploadRequest {
            key,
            upload_id,
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

impl<'de> Deserialize<'de> for SyntheticsTestFileAbortMultipartUploadRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTestFileAbortMultipartUploadRequestVisitor;
        impl<'a> Visitor<'a> for SyntheticsTestFileAbortMultipartUploadRequestVisitor {
            type Value = SyntheticsTestFileAbortMultipartUploadRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut key: Option<String> = None;
                let mut upload_id: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "key" => {
                            key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "uploadId" => {
                            upload_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let key = key.ok_or_else(|| M::Error::missing_field("key"))?;
                let upload_id = upload_id.ok_or_else(|| M::Error::missing_field("upload_id"))?;

                let content = SyntheticsTestFileAbortMultipartUploadRequest {
                    key,
                    upload_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTestFileAbortMultipartUploadRequestVisitor)
    }
}

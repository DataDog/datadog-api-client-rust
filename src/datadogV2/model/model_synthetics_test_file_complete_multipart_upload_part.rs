// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A completed part of a multipart upload.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTestFileCompleteMultipartUploadPart {
    /// The ETag returned by the storage provider after uploading the part.
    #[serde(rename = "ETag")]
    pub e_tag: String,
    /// The 1-indexed part number for the multipart upload.
    #[serde(rename = "PartNumber")]
    pub part_number: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTestFileCompleteMultipartUploadPart {
    pub fn new(e_tag: String, part_number: i64) -> SyntheticsTestFileCompleteMultipartUploadPart {
        SyntheticsTestFileCompleteMultipartUploadPart {
            e_tag,
            part_number,
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

impl<'de> Deserialize<'de> for SyntheticsTestFileCompleteMultipartUploadPart {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTestFileCompleteMultipartUploadPartVisitor;
        impl<'a> Visitor<'a> for SyntheticsTestFileCompleteMultipartUploadPartVisitor {
            type Value = SyntheticsTestFileCompleteMultipartUploadPart;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut e_tag: Option<String> = None;
                let mut part_number: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "ETag" => {
                            e_tag = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "PartNumber" => {
                            part_number =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let e_tag = e_tag.ok_or_else(|| M::Error::missing_field("e_tag"))?;
                let part_number =
                    part_number.ok_or_else(|| M::Error::missing_field("part_number"))?;

                let content = SyntheticsTestFileCompleteMultipartUploadPart {
                    e_tag,
                    part_number,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTestFileCompleteMultipartUploadPartVisitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A part descriptor for initiating a multipart upload.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTestFileMultipartPresignedUrlsPart {
    /// Base64-encoded MD5 digest of the part content.
    #[serde(rename = "md5")]
    pub md5: String,
    /// The 1-indexed part number for the multipart upload.
    #[serde(rename = "partNumber")]
    pub part_number: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTestFileMultipartPresignedUrlsPart {
    pub fn new(md5: String, part_number: i64) -> SyntheticsTestFileMultipartPresignedUrlsPart {
        SyntheticsTestFileMultipartPresignedUrlsPart {
            md5,
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

impl<'de> Deserialize<'de> for SyntheticsTestFileMultipartPresignedUrlsPart {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTestFileMultipartPresignedUrlsPartVisitor;
        impl<'a> Visitor<'a> for SyntheticsTestFileMultipartPresignedUrlsPartVisitor {
            type Value = SyntheticsTestFileMultipartPresignedUrlsPart;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut md5: Option<String> = None;
                let mut part_number: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "md5" => {
                            md5 = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "partNumber" => {
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
                let md5 = md5.ok_or_else(|| M::Error::missing_field("md5"))?;
                let part_number =
                    part_number.ok_or_else(|| M::Error::missing_field("part_number"))?;

                let content = SyntheticsTestFileMultipartPresignedUrlsPart {
                    md5,
                    part_number,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTestFileMultipartPresignedUrlsPartVisitor)
    }
}

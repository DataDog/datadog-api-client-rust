// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object describing a file to be used as part of the request in the test.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTestRequestBodyFile {
    /// Bucket key of the file.
    #[serde(rename = "bucketKey")]
    pub bucket_key: Option<String>,
    /// Content of the file.
    #[serde(rename = "content")]
    pub content: Option<String>,
    /// Name of the file.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Size of the file.
    #[serde(rename = "size")]
    pub size: Option<i64>,
    /// Type of the file.
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTestRequestBodyFile {
    pub fn new() -> SyntheticsTestRequestBodyFile {
        SyntheticsTestRequestBodyFile {
            bucket_key: None,
            content: None,
            name: None,
            size: None,
            type_: None,
            _unparsed: false,
        }
    }

    pub fn bucket_key(mut self, value: String) -> Self {
        self.bucket_key = Some(value);
        self
    }

    pub fn content(mut self, value: String) -> Self {
        self.content = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn size(mut self, value: i64) -> Self {
        self.size = Some(value);
        self
    }

    pub fn type_(mut self, value: String) -> Self {
        self.type_ = Some(value);
        self
    }
}

impl Default for SyntheticsTestRequestBodyFile {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsTestRequestBodyFile {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTestRequestBodyFileVisitor;
        impl<'a> Visitor<'a> for SyntheticsTestRequestBodyFileVisitor {
            type Value = SyntheticsTestRequestBodyFile;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut bucket_key: Option<String> = None;
                let mut content: Option<String> = None;
                let mut name: Option<String> = None;
                let mut size: Option<i64> = None;
                let mut type_: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "bucketKey" => {
                            if v.is_null() {
                                continue;
                            }
                            bucket_key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "content" => {
                            if v.is_null() {
                                continue;
                            }
                            content = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "size" => {
                            if v.is_null() {
                                continue;
                            }
                            size = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SyntheticsTestRequestBodyFile {
                    bucket_key,
                    content,
                    name,
                    size,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTestRequestBodyFileVisitor)
    }
}

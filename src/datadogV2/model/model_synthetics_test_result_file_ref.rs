// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Reference to a file attached to a Synthetic test request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTestResultFileRef {
    /// Storage bucket key where the file is stored.
    #[serde(rename = "bucket_key")]
    pub bucket_key: Option<String>,
    /// Encoding of the file contents.
    #[serde(rename = "encoding")]
    pub encoding: Option<String>,
    /// File name.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// File size in bytes.
    #[serde(rename = "size")]
    pub size: Option<i64>,
    /// File MIME type.
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTestResultFileRef {
    pub fn new() -> SyntheticsTestResultFileRef {
        SyntheticsTestResultFileRef {
            bucket_key: None,
            encoding: None,
            name: None,
            size: None,
            type_: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn bucket_key(mut self, value: String) -> Self {
        self.bucket_key = Some(value);
        self
    }

    pub fn encoding(mut self, value: String) -> Self {
        self.encoding = Some(value);
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

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl Default for SyntheticsTestResultFileRef {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsTestResultFileRef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTestResultFileRefVisitor;
        impl<'a> Visitor<'a> for SyntheticsTestResultFileRefVisitor {
            type Value = SyntheticsTestResultFileRef;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut bucket_key: Option<String> = None;
                let mut encoding: Option<String> = None;
                let mut name: Option<String> = None;
                let mut size: Option<i64> = None;
                let mut type_: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "bucket_key" => {
                            if v.is_null() {
                                continue;
                            }
                            bucket_key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "encoding" => {
                            if v.is_null() {
                                continue;
                            }
                            encoding = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsTestResultFileRef {
                    bucket_key,
                    encoding,
                    name,
                    size,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTestResultFileRefVisitor)
    }
}

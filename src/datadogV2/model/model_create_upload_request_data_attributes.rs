// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `CreateUploadRequestDataAttributes` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CreateUploadRequestDataAttributes {
    /// The headers of the file to upload.
    #[serde(rename = "headers")]
    pub headers: Vec<String>,
    /// The number of parts in the upload.
    #[serde(rename = "part_count")]
    pub part_count: i32,
    /// The size of each part in the upload in bytes. For multipart uploads (part_count > 1), all parts except the last one must be at least 5,000,000 bytes. For single-part uploads (part_count = 1), any size is allowed.
    #[serde(rename = "part_size")]
    pub part_size: i64,
    /// The name of the reference table.
    #[serde(rename = "table_name")]
    pub table_name: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CreateUploadRequestDataAttributes {
    pub fn new(
        headers: Vec<String>,
        part_count: i32,
        part_size: i64,
        table_name: String,
    ) -> CreateUploadRequestDataAttributes {
        CreateUploadRequestDataAttributes {
            headers,
            part_count,
            part_size,
            table_name,
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

impl<'de> Deserialize<'de> for CreateUploadRequestDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CreateUploadRequestDataAttributesVisitor;
        impl<'a> Visitor<'a> for CreateUploadRequestDataAttributesVisitor {
            type Value = CreateUploadRequestDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut headers: Option<Vec<String>> = None;
                let mut part_count: Option<i32> = None;
                let mut part_size: Option<i64> = None;
                let mut table_name: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "headers" => {
                            headers = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "part_count" => {
                            part_count = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "part_size" => {
                            part_size = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "table_name" => {
                            table_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let headers = headers.ok_or_else(|| M::Error::missing_field("headers"))?;
                let part_count = part_count.ok_or_else(|| M::Error::missing_field("part_count"))?;
                let part_size = part_size.ok_or_else(|| M::Error::missing_field("part_size"))?;
                let table_name = table_name.ok_or_else(|| M::Error::missing_field("table_name"))?;

                let content = CreateUploadRequestDataAttributes {
                    headers,
                    part_count,
                    part_size,
                    table_name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CreateUploadRequestDataAttributesVisitor)
    }
}

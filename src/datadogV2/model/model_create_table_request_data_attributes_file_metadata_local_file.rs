// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Local file metadata for create requests using the upload ID.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CreateTableRequestDataAttributesFileMetadataLocalFile {
    /// The upload ID.
    #[serde(rename = "upload_id")]
    pub upload_id: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CreateTableRequestDataAttributesFileMetadataLocalFile {
    pub fn new(upload_id: String) -> CreateTableRequestDataAttributesFileMetadataLocalFile {
        CreateTableRequestDataAttributesFileMetadataLocalFile {
            upload_id,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for CreateTableRequestDataAttributesFileMetadataLocalFile {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CreateTableRequestDataAttributesFileMetadataLocalFileVisitor;
        impl<'a> Visitor<'a> for CreateTableRequestDataAttributesFileMetadataLocalFileVisitor {
            type Value = CreateTableRequestDataAttributesFileMetadataLocalFile;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut upload_id: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "upload_id" => {
                            upload_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let upload_id = upload_id.ok_or_else(|| M::Error::missing_field("upload_id"))?;

                let content = CreateTableRequestDataAttributesFileMetadataLocalFile {
                    upload_id,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CreateTableRequestDataAttributesFileMetadataLocalFileVisitor)
    }
}

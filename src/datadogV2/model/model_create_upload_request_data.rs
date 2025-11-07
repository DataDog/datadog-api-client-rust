// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Request data for creating an upload for a file to be ingested into a reference table.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CreateUploadRequestData {
    /// Upload configuration specifying how data is uploaded by the user, and properties of the table to associate the upload with.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::CreateUploadRequestDataAttributes>,
    /// Upload resource type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::CreateUploadRequestDataType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CreateUploadRequestData {
    pub fn new(
        type_: crate::datadogV2::model::CreateUploadRequestDataType,
    ) -> CreateUploadRequestData {
        CreateUploadRequestData {
            attributes: None,
            type_,
            _unparsed: false,
        }
    }

    pub fn attributes(
        mut self,
        value: crate::datadogV2::model::CreateUploadRequestDataAttributes,
    ) -> Self {
        self.attributes = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for CreateUploadRequestData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CreateUploadRequestDataVisitor;
        impl<'a> Visitor<'a> for CreateUploadRequestDataVisitor {
            type Value = CreateUploadRequestData;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attributes: Option<
                    crate::datadogV2::model::CreateUploadRequestDataAttributes,
                > = None;
                let mut type_: Option<crate::datadogV2::model::CreateUploadRequestDataType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "attributes" => {
                            if v.is_null() {
                                continue;
                            }
                            attributes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::CreateUploadRequestDataType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = CreateUploadRequestData {
                    attributes,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CreateUploadRequestDataVisitor)
    }
}

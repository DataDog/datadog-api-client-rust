// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The S3 archive destination.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LogsArchiveDestinationS3 {
    /// The bucket where the archive will be stored.
    #[serde(rename = "bucket")]
    pub bucket: String,
    /// The S3 encryption settings.
    #[serde(rename = "encryption")]
    pub encryption: Option<crate::datadogV2::model::LogsArchiveEncryptionS3>,
    /// The S3 Archive's integration destination.
    #[serde(rename = "integration")]
    pub integration: crate::datadogV2::model::LogsArchiveIntegrationS3,
    /// The archive path.
    #[serde(rename = "path")]
    pub path: Option<String>,
    /// The storage class where the archive will be stored.
    #[serde(rename = "storage_class")]
    pub storage_class: Option<crate::datadogV2::model::LogsArchiveStorageClassS3Type>,
    /// Type of the S3 archive destination.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::LogsArchiveDestinationS3Type,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LogsArchiveDestinationS3 {
    pub fn new(
        bucket: String,
        integration: crate::datadogV2::model::LogsArchiveIntegrationS3,
        type_: crate::datadogV2::model::LogsArchiveDestinationS3Type,
    ) -> LogsArchiveDestinationS3 {
        LogsArchiveDestinationS3 {
            bucket,
            encryption: None,
            integration,
            path: None,
            storage_class: None,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn encryption(mut self, value: crate::datadogV2::model::LogsArchiveEncryptionS3) -> Self {
        self.encryption = Some(value);
        self
    }

    pub fn path(mut self, value: String) -> Self {
        self.path = Some(value);
        self
    }

    pub fn storage_class(
        mut self,
        value: crate::datadogV2::model::LogsArchiveStorageClassS3Type,
    ) -> Self {
        self.storage_class = Some(value);
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

impl<'de> Deserialize<'de> for LogsArchiveDestinationS3 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogsArchiveDestinationS3Visitor;
        impl<'a> Visitor<'a> for LogsArchiveDestinationS3Visitor {
            type Value = LogsArchiveDestinationS3;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut bucket: Option<String> = None;
                let mut encryption: Option<crate::datadogV2::model::LogsArchiveEncryptionS3> = None;
                let mut integration: Option<crate::datadogV2::model::LogsArchiveIntegrationS3> =
                    None;
                let mut path: Option<String> = None;
                let mut storage_class: Option<
                    crate::datadogV2::model::LogsArchiveStorageClassS3Type,
                > = None;
                let mut type_: Option<crate::datadogV2::model::LogsArchiveDestinationS3Type> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "bucket" => {
                            bucket = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "encryption" => {
                            if v.is_null() {
                                continue;
                            }
                            encryption = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "integration" => {
                            integration =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "path" => {
                            if v.is_null() {
                                continue;
                            }
                            path = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "storage_class" => {
                            if v.is_null() {
                                continue;
                            }
                            storage_class =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _storage_class) = storage_class {
                                match _storage_class {
                                    crate::datadogV2::model::LogsArchiveStorageClassS3Type::UnparsedObject(_storage_class) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::LogsArchiveDestinationS3Type::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let bucket = bucket.ok_or_else(|| M::Error::missing_field("bucket"))?;
                let integration =
                    integration.ok_or_else(|| M::Error::missing_field("integration"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = LogsArchiveDestinationS3 {
                    bucket,
                    encryption,
                    integration,
                    path,
                    storage_class,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogsArchiveDestinationS3Visitor)
    }
}

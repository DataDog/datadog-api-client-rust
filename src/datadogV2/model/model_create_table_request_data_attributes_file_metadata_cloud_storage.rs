// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Cloud storage file metadata for create requests. Both access_details and sync_enabled are required.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CreateTableRequestDataAttributesFileMetadataCloudStorage {
    /// Cloud storage access configuration for the reference table data file.
    #[serde(rename = "access_details")]
    pub access_details:
        crate::datadogV2::model::CreateTableRequestDataAttributesFileMetadataOneOfAccessDetails,
    /// Whether this table is synced automatically.
    #[serde(rename = "sync_enabled")]
    pub sync_enabled: bool,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CreateTableRequestDataAttributesFileMetadataCloudStorage {
    pub fn new(
        access_details: crate::datadogV2::model::CreateTableRequestDataAttributesFileMetadataOneOfAccessDetails,
        sync_enabled: bool,
    ) -> CreateTableRequestDataAttributesFileMetadataCloudStorage {
        CreateTableRequestDataAttributesFileMetadataCloudStorage {
            access_details,
            sync_enabled,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for CreateTableRequestDataAttributesFileMetadataCloudStorage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CreateTableRequestDataAttributesFileMetadataCloudStorageVisitor;
        impl<'a> Visitor<'a> for CreateTableRequestDataAttributesFileMetadataCloudStorageVisitor {
            type Value = CreateTableRequestDataAttributesFileMetadataCloudStorage;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut access_details: Option<crate::datadogV2::model::CreateTableRequestDataAttributesFileMetadataOneOfAccessDetails> = None;
                let mut sync_enabled: Option<bool> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "access_details" => {
                            access_details =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sync_enabled" => {
                            sync_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let access_details =
                    access_details.ok_or_else(|| M::Error::missing_field("access_details"))?;
                let sync_enabled =
                    sync_enabled.ok_or_else(|| M::Error::missing_field("sync_enabled"))?;

                let content = CreateTableRequestDataAttributesFileMetadataCloudStorage {
                    access_details,
                    sync_enabled,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(CreateTableRequestDataAttributesFileMetadataCloudStorageVisitor)
    }
}

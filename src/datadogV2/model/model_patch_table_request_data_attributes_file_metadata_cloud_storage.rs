// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Cloud storage file metadata for patch requests. Allows partial updates of access_details and sync_enabled.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PatchTableRequestDataAttributesFileMetadataCloudStorage {
    /// Cloud storage access configuration for the reference table data file.
    #[serde(rename = "access_details")]
    pub access_details: Option<
        crate::datadogV2::model::PatchTableRequestDataAttributesFileMetadataOneOfAccessDetails,
    >,
    /// Whether this table is synced automatically.
    #[serde(rename = "sync_enabled")]
    pub sync_enabled: Option<bool>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl PatchTableRequestDataAttributesFileMetadataCloudStorage {
    pub fn new() -> PatchTableRequestDataAttributesFileMetadataCloudStorage {
        PatchTableRequestDataAttributesFileMetadataCloudStorage {
            access_details: None,
            sync_enabled: None,
            _unparsed: false,
        }
    }

    pub fn access_details(
        mut self,
        value: crate::datadogV2::model::PatchTableRequestDataAttributesFileMetadataOneOfAccessDetails,
    ) -> Self {
        self.access_details = Some(value);
        self
    }

    pub fn sync_enabled(mut self, value: bool) -> Self {
        self.sync_enabled = Some(value);
        self
    }
}

impl Default for PatchTableRequestDataAttributesFileMetadataCloudStorage {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for PatchTableRequestDataAttributesFileMetadataCloudStorage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PatchTableRequestDataAttributesFileMetadataCloudStorageVisitor;
        impl<'a> Visitor<'a> for PatchTableRequestDataAttributesFileMetadataCloudStorageVisitor {
            type Value = PatchTableRequestDataAttributesFileMetadataCloudStorage;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut access_details: Option<crate::datadogV2::model::PatchTableRequestDataAttributesFileMetadataOneOfAccessDetails> = None;
                let mut sync_enabled: Option<bool> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "access_details" => {
                            if v.is_null() {
                                continue;
                            }
                            access_details =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sync_enabled" => {
                            if v.is_null() {
                                continue;
                            }
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

                let content = PatchTableRequestDataAttributesFileMetadataCloudStorage {
                    access_details,
                    sync_enabled,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(PatchTableRequestDataAttributesFileMetadataCloudStorageVisitor)
    }
}

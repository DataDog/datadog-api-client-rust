// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The GCS archive destination.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LogsArchiveDestinationGCS {
    /// The bucket where the archive will be stored.
    #[serde(rename = "bucket")]
    pub bucket: String,
    /// The GCS archive's integration destination.
    #[serde(rename = "integration")]
    pub integration: crate::datadogV2::model::LogsArchiveIntegrationGCS,
    /// The archive path.
    #[serde(rename = "path")]
    pub path: Option<String>,
    /// Type of the GCS archive destination.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::LogsArchiveDestinationGCSType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LogsArchiveDestinationGCS {
    pub fn new(
        bucket: String,
        integration: crate::datadogV2::model::LogsArchiveIntegrationGCS,
        type_: crate::datadogV2::model::LogsArchiveDestinationGCSType,
    ) -> LogsArchiveDestinationGCS {
        LogsArchiveDestinationGCS {
            bucket,
            integration,
            path: None,
            type_,
            _unparsed: false,
        }
    }

    pub fn path(mut self, value: String) -> Self {
        self.path = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for LogsArchiveDestinationGCS {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogsArchiveDestinationGCSVisitor;
        impl<'a> Visitor<'a> for LogsArchiveDestinationGCSVisitor {
            type Value = LogsArchiveDestinationGCS;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut bucket: Option<String> = None;
                let mut integration: Option<crate::datadogV2::model::LogsArchiveIntegrationGCS> =
                    None;
                let mut path: Option<String> = None;
                let mut type_: Option<crate::datadogV2::model::LogsArchiveDestinationGCSType> =
                    None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "bucket" => {
                            bucket = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::LogsArchiveDestinationGCSType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }
                let bucket = bucket.ok_or_else(|| M::Error::missing_field("bucket"))?;
                let integration =
                    integration.ok_or_else(|| M::Error::missing_field("integration"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = LogsArchiveDestinationGCS {
                    bucket,
                    integration,
                    path,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogsArchiveDestinationGCSVisitor)
    }
}

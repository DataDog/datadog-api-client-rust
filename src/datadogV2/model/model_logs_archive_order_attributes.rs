// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes associated with the archive order.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LogsArchiveOrderAttributes {
    /// An ordered array of `<ARCHIVE_ID>` strings, the order of archive IDs in the array
    /// define the overall archives order for Datadog.
    #[serde(rename = "archive_ids")]
    pub archive_ids: Vec<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LogsArchiveOrderAttributes {
    pub fn new(archive_ids: Vec<String>) -> LogsArchiveOrderAttributes {
        LogsArchiveOrderAttributes {
            archive_ids,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for LogsArchiveOrderAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogsArchiveOrderAttributesVisitor;
        impl<'a> Visitor<'a> for LogsArchiveOrderAttributesVisitor {
            type Value = LogsArchiveOrderAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut archive_ids: Option<Vec<String>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "archive_ids" => {
                            archive_ids =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let archive_ids =
                    archive_ids.ok_or_else(|| M::Error::missing_field("archive_ids"))?;

                let content = LogsArchiveOrderAttributes {
                    archive_ids,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogsArchiveOrderAttributesVisitor)
    }
}

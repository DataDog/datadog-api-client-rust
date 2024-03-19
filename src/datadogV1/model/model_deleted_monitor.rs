// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Response from the delete monitor call.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DeletedMonitor {
    /// ID of the deleted monitor.
    #[serde(rename = "deleted_monitor_id")]
    pub deleted_monitor_id: Option<i64>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DeletedMonitor {
    pub fn new() -> DeletedMonitor {
        DeletedMonitor {
            deleted_monitor_id: None,
            _unparsed: false,
        }
    }

    pub fn deleted_monitor_id(mut self, value: i64) -> Self {
        self.deleted_monitor_id = Some(value);
        self
    }
}

impl Default for DeletedMonitor {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for DeletedMonitor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DeletedMonitorVisitor;
        impl<'a> Visitor<'a> for DeletedMonitorVisitor {
            type Value = DeletedMonitor;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut deleted_monitor_id: Option<i64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "deleted_monitor_id" => {
                            if v.is_null() {
                                continue;
                            }
                            deleted_monitor_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = DeletedMonitor {
                    deleted_monitor_id,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DeletedMonitorVisitor)
    }
}

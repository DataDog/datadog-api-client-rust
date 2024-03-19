// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The monitor identified by the downtime.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DowntimeRelationshipsMonitor {
    /// Data for the monitor.
    #[serde(rename = "data", default, with = "::serde_with::rust::double_option")]
    pub data: Option<Option<crate::datadogV2::model::DowntimeRelationshipsMonitorData>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DowntimeRelationshipsMonitor {
    pub fn new() -> DowntimeRelationshipsMonitor {
        DowntimeRelationshipsMonitor {
            data: None,
            _unparsed: false,
        }
    }

    pub fn data(
        mut self,
        value: Option<crate::datadogV2::model::DowntimeRelationshipsMonitorData>,
    ) -> Self {
        self.data = Some(value);
        self
    }
}

impl Default for DowntimeRelationshipsMonitor {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for DowntimeRelationshipsMonitor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DowntimeRelationshipsMonitorVisitor;
        impl<'a> Visitor<'a> for DowntimeRelationshipsMonitorVisitor {
            type Value = DowntimeRelationshipsMonitor;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data: Option<
                    Option<crate::datadogV2::model::DowntimeRelationshipsMonitorData>,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data" => {
                            data = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = DowntimeRelationshipsMonitor { data, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DowntimeRelationshipsMonitorVisitor)
    }
}

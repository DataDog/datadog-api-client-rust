// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object of the monitor identifier.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DowntimeMonitorIdentifierId {
    /// ID of the monitor to prevent notifications.
    #[serde(rename = "monitor_id")]
    pub monitor_id: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DowntimeMonitorIdentifierId {
    pub fn new(monitor_id: i64) -> DowntimeMonitorIdentifierId {
        DowntimeMonitorIdentifierId {
            monitor_id,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for DowntimeMonitorIdentifierId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DowntimeMonitorIdentifierIdVisitor;
        impl<'a> Visitor<'a> for DowntimeMonitorIdentifierIdVisitor {
            type Value = DowntimeMonitorIdentifierId;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut monitor_id: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "monitor_id" => {
                            monitor_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let monitor_id = monitor_id.ok_or_else(|| M::Error::missing_field("monitor_id"))?;

                let content = DowntimeMonitorIdentifierId {
                    monitor_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DowntimeMonitorIdentifierIdVisitor)
    }
}

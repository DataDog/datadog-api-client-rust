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
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DowntimeRelationshipsMonitor {
    pub fn new() -> DowntimeRelationshipsMonitor {
        DowntimeRelationshipsMonitor {
            data: None,
            additional_properties: std::collections::BTreeMap::new(),
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

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
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
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data" => {
                            data = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = DowntimeRelationshipsMonitor {
                    data,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DowntimeRelationshipsMonitorVisitor)
    }
}

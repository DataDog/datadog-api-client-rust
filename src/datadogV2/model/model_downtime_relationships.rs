// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// All relationships associated with downtime.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DowntimeRelationships {
    /// The user who created the downtime.
    #[serde(rename = "created_by")]
    pub created_by: Option<crate::datadogV2::model::DowntimeRelationshipsCreatedBy>,
    /// The monitor identified by the downtime.
    #[serde(rename = "monitor")]
    pub monitor: Option<crate::datadogV2::model::DowntimeRelationshipsMonitor>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DowntimeRelationships {
    pub fn new() -> DowntimeRelationships {
        DowntimeRelationships {
            created_by: None,
            monitor: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn created_by(
        mut self,
        value: crate::datadogV2::model::DowntimeRelationshipsCreatedBy,
    ) -> Self {
        self.created_by = Some(value);
        self
    }

    pub fn monitor(mut self, value: crate::datadogV2::model::DowntimeRelationshipsMonitor) -> Self {
        self.monitor = Some(value);
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

impl Default for DowntimeRelationships {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for DowntimeRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DowntimeRelationshipsVisitor;
        impl<'a> Visitor<'a> for DowntimeRelationshipsVisitor {
            type Value = DowntimeRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_by: Option<
                    crate::datadogV2::model::DowntimeRelationshipsCreatedBy,
                > = None;
                let mut monitor: Option<crate::datadogV2::model::DowntimeRelationshipsMonitor> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created_by" => {
                            if v.is_null() {
                                continue;
                            }
                            created_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "monitor" => {
                            if v.is_null() {
                                continue;
                            }
                            monitor = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = DowntimeRelationships {
                    created_by,
                    monitor,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DowntimeRelationshipsVisitor)
    }
}

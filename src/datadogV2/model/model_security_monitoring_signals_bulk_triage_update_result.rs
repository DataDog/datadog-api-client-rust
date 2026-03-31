// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The result payload of a bulk signal triage update.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringSignalsBulkTriageUpdateResult {
    /// The number of signals updated.
    #[serde(rename = "count")]
    pub count: i64,
    /// The list of updated signals.
    #[serde(rename = "events")]
    pub events: Vec<crate::datadogV2::model::SecurityMonitoringSignalsBulkTriageEvent>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringSignalsBulkTriageUpdateResult {
    pub fn new(
        count: i64,
        events: Vec<crate::datadogV2::model::SecurityMonitoringSignalsBulkTriageEvent>,
    ) -> SecurityMonitoringSignalsBulkTriageUpdateResult {
        SecurityMonitoringSignalsBulkTriageUpdateResult {
            count,
            events,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for SecurityMonitoringSignalsBulkTriageUpdateResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringSignalsBulkTriageUpdateResultVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringSignalsBulkTriageUpdateResultVisitor {
            type Value = SecurityMonitoringSignalsBulkTriageUpdateResult;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut count: Option<i64> = None;
                let mut events: Option<
                    Vec<crate::datadogV2::model::SecurityMonitoringSignalsBulkTriageEvent>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "count" => {
                            count = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "events" => {
                            events = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let count = count.ok_or_else(|| M::Error::missing_field("count"))?;
                let events = events.ok_or_else(|| M::Error::missing_field("events"))?;

                let content = SecurityMonitoringSignalsBulkTriageUpdateResult {
                    count,
                    events,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringSignalsBulkTriageUpdateResultVisitor)
    }
}

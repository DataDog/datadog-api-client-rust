// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes describing which incident to add the signal to.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AddSignalToIncidentRequest {
    /// Whether to post the signal on the incident timeline.
    #[serde(rename = "add_to_signal_timeline")]
    pub add_to_signal_timeline: Option<bool>,
    /// Public ID attribute of the incident to which the signal will be added.
    #[serde(rename = "incident_id")]
    pub incident_id: i64,
    /// Version of the updated signal. If server side version is higher, update will be rejected.
    #[serde(rename = "version")]
    pub version: Option<i64>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AddSignalToIncidentRequest {
    pub fn new(incident_id: i64) -> AddSignalToIncidentRequest {
        AddSignalToIncidentRequest {
            add_to_signal_timeline: None,
            incident_id,
            version: None,
            _unparsed: false,
        }
    }

    pub fn add_to_signal_timeline(mut self, value: bool) -> Self {
        self.add_to_signal_timeline = Some(value);
        self
    }

    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for AddSignalToIncidentRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AddSignalToIncidentRequestVisitor;
        impl<'a> Visitor<'a> for AddSignalToIncidentRequestVisitor {
            type Value = AddSignalToIncidentRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut add_to_signal_timeline: Option<bool> = None;
                let mut incident_id: Option<i64> = None;
                let mut version: Option<i64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "add_to_signal_timeline" => {
                            if v.is_null() {
                                continue;
                            }
                            add_to_signal_timeline =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "incident_id" => {
                            incident_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "version" => {
                            if v.is_null() {
                                continue;
                            }
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let incident_id =
                    incident_id.ok_or_else(|| M::Error::missing_field("incident_id"))?;

                let content = AddSignalToIncidentRequest {
                    add_to_signal_timeline,
                    incident_id,
                    version,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AddSignalToIncidentRequestVisitor)
    }
}

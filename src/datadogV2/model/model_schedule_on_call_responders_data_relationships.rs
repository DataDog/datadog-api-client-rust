// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Relationships for a schedule's on-call responders lookup, including the schedule and its responder groups.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ScheduleOnCallRespondersDataRelationships {
    /// Defines the list of per-position (previous, current, next) responder groups for the schedule.
    #[serde(rename = "responders")]
    pub responders:
        Option<crate::datadogV2::model::ScheduleOnCallRespondersDataRelationshipsResponders>,
    /// Defines the relationship to the schedule this on-call responders lookup was performed for.
    #[serde(rename = "schedule")]
    pub schedule:
        Option<crate::datadogV2::model::ScheduleOnCallRespondersDataRelationshipsSchedule>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ScheduleOnCallRespondersDataRelationships {
    pub fn new() -> ScheduleOnCallRespondersDataRelationships {
        ScheduleOnCallRespondersDataRelationships {
            responders: None,
            schedule: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn responders(
        mut self,
        value: crate::datadogV2::model::ScheduleOnCallRespondersDataRelationshipsResponders,
    ) -> Self {
        self.responders = Some(value);
        self
    }

    pub fn schedule(
        mut self,
        value: crate::datadogV2::model::ScheduleOnCallRespondersDataRelationshipsSchedule,
    ) -> Self {
        self.schedule = Some(value);
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

impl Default for ScheduleOnCallRespondersDataRelationships {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ScheduleOnCallRespondersDataRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ScheduleOnCallRespondersDataRelationshipsVisitor;
        impl<'a> Visitor<'a> for ScheduleOnCallRespondersDataRelationshipsVisitor {
            type Value = ScheduleOnCallRespondersDataRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut responders: Option<
                    crate::datadogV2::model::ScheduleOnCallRespondersDataRelationshipsResponders,
                > = None;
                let mut schedule: Option<
                    crate::datadogV2::model::ScheduleOnCallRespondersDataRelationshipsSchedule,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "responders" => {
                            if v.is_null() {
                                continue;
                            }
                            responders = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "schedule" => {
                            if v.is_null() {
                                continue;
                            }
                            schedule = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ScheduleOnCallRespondersDataRelationships {
                    responders,
                    schedule,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ScheduleOnCallRespondersDataRelationshipsVisitor)
    }
}

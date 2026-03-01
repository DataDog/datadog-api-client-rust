// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Trigger a workflow from an Incident Schedule. The workflow must be published.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentScheduleTrigger {
    /// Incident type filter for the schedule.
    #[serde(rename = "incidentType")]
    pub incident_type: Option<String>,
    /// Recurrence rule expression for scheduling.
    #[serde(rename = "rrule")]
    pub rrule: String,
    /// A condition evaluated against incident tags.
    #[serde(rename = "tagCondition")]
    pub tag_condition: Option<crate::datadogV2::model::IncidentCondition>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentScheduleTrigger {
    pub fn new(rrule: String) -> IncidentScheduleTrigger {
        IncidentScheduleTrigger {
            incident_type: None,
            rrule,
            tag_condition: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn incident_type(mut self, value: String) -> Self {
        self.incident_type = Some(value);
        self
    }

    pub fn tag_condition(mut self, value: crate::datadogV2::model::IncidentCondition) -> Self {
        self.tag_condition = Some(value);
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

impl<'de> Deserialize<'de> for IncidentScheduleTrigger {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentScheduleTriggerVisitor;
        impl<'a> Visitor<'a> for IncidentScheduleTriggerVisitor {
            type Value = IncidentScheduleTrigger;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut incident_type: Option<String> = None;
                let mut rrule: Option<String> = None;
                let mut tag_condition: Option<crate::datadogV2::model::IncidentCondition> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "incidentType" => {
                            if v.is_null() {
                                continue;
                            }
                            incident_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rrule" => {
                            rrule = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tagCondition" => {
                            if v.is_null() {
                                continue;
                            }
                            tag_condition =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let rrule = rrule.ok_or_else(|| M::Error::missing_field("rrule"))?;

                let content = IncidentScheduleTrigger {
                    incident_type,
                    rrule,
                    tag_condition,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentScheduleTriggerVisitor)
    }
}

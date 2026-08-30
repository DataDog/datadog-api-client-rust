// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Trigger a workflow when an incident is declared or updated.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentSavedTrigger {
    /// The maximum number of times to execute a workflow for an incident.
    #[serde(rename = "executionLimit")]
    pub execution_limit: Option<crate::datadogV2::model::ExecutionLimit>,
    /// The type of incident that triggers the workflow.
    #[serde(rename = "incidentType")]
    pub incident_type: Option<String>,
    /// Whether to execute the workflow serially for an incident.
    #[serde(rename = "serialExecution")]
    pub serial_execution: Option<crate::datadogV2::model::SerialExecution>,
    /// Conditions that determine which incidents trigger the workflow.
    #[serde(rename = "tagCondition")]
    pub tag_condition: Option<crate::datadogV2::model::IncidentCondition>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentSavedTrigger {
    pub fn new() -> IncidentSavedTrigger {
        IncidentSavedTrigger {
            execution_limit: None,
            incident_type: None,
            serial_execution: None,
            tag_condition: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn execution_limit(mut self, value: crate::datadogV2::model::ExecutionLimit) -> Self {
        self.execution_limit = Some(value);
        self
    }

    pub fn incident_type(mut self, value: String) -> Self {
        self.incident_type = Some(value);
        self
    }

    pub fn serial_execution(mut self, value: crate::datadogV2::model::SerialExecution) -> Self {
        self.serial_execution = Some(value);
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

impl Default for IncidentSavedTrigger {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for IncidentSavedTrigger {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentSavedTriggerVisitor;
        impl<'a> Visitor<'a> for IncidentSavedTriggerVisitor {
            type Value = IncidentSavedTrigger;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut execution_limit: Option<crate::datadogV2::model::ExecutionLimit> = None;
                let mut incident_type: Option<String> = None;
                let mut serial_execution: Option<crate::datadogV2::model::SerialExecution> = None;
                let mut tag_condition: Option<crate::datadogV2::model::IncidentCondition> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "executionLimit" => {
                            if v.is_null() {
                                continue;
                            }
                            execution_limit =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "incidentType" => {
                            if v.is_null() {
                                continue;
                            }
                            incident_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "serialExecution" => {
                            if v.is_null() {
                                continue;
                            }
                            serial_execution =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = IncidentSavedTrigger {
                    execution_limit,
                    incident_type,
                    serial_execution,
                    tag_condition,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentSavedTriggerVisitor)
    }
}

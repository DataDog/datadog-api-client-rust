// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Options on sequence detection method.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringRuleSequenceDetectionOptions {
    /// Transitions defining the allowed order of steps and their evaluation windows.
    #[serde(rename = "stepTransitions")]
    pub step_transitions:
        Option<Vec<crate::datadogV2::model::SecurityMonitoringRuleSequenceDetectionStepTransition>>,
    /// Steps that define the conditions to be matched in sequence.
    #[serde(rename = "steps")]
    pub steps: Option<Vec<crate::datadogV2::model::SecurityMonitoringRuleSequenceDetectionStep>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringRuleSequenceDetectionOptions {
    pub fn new() -> SecurityMonitoringRuleSequenceDetectionOptions {
        SecurityMonitoringRuleSequenceDetectionOptions {
            step_transitions: None,
            steps: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn step_transitions(
        mut self,
        value: Vec<crate::datadogV2::model::SecurityMonitoringRuleSequenceDetectionStepTransition>,
    ) -> Self {
        self.step_transitions = Some(value);
        self
    }

    pub fn steps(
        mut self,
        value: Vec<crate::datadogV2::model::SecurityMonitoringRuleSequenceDetectionStep>,
    ) -> Self {
        self.steps = Some(value);
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

impl Default for SecurityMonitoringRuleSequenceDetectionOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SecurityMonitoringRuleSequenceDetectionOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringRuleSequenceDetectionOptionsVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringRuleSequenceDetectionOptionsVisitor {
            type Value = SecurityMonitoringRuleSequenceDetectionOptions;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut step_transitions: Option<Vec<crate::datadogV2::model::SecurityMonitoringRuleSequenceDetectionStepTransition>> = None;
                let mut steps: Option<
                    Vec<crate::datadogV2::model::SecurityMonitoringRuleSequenceDetectionStep>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "stepTransitions" => {
                            if v.is_null() {
                                continue;
                            }
                            step_transitions =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "steps" => {
                            if v.is_null() {
                                continue;
                            }
                            steps = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SecurityMonitoringRuleSequenceDetectionOptions {
                    step_transitions,
                    steps,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringRuleSequenceDetectionOptionsVisitor)
    }
}

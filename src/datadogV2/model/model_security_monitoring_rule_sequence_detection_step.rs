// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Step definition for sequence detection containing the step name, condition, and evaluation window.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringRuleSequenceDetectionStep {
    /// Condition referencing rule queries (e.g., `a > 0`).
    #[serde(rename = "condition")]
    pub condition: Option<String>,
    /// A time window is specified to match when at least one of the cases matches true. This is a sliding window
    /// and evaluates in real time. For third party detection method, this field is not used.
    #[serde(rename = "evaluationWindow")]
    pub evaluation_window: Option<crate::datadogV2::model::SecurityMonitoringRuleEvaluationWindow>,
    /// Unique name identifying the step.
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringRuleSequenceDetectionStep {
    pub fn new() -> SecurityMonitoringRuleSequenceDetectionStep {
        SecurityMonitoringRuleSequenceDetectionStep {
            condition: None,
            evaluation_window: None,
            name: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn condition(mut self, value: String) -> Self {
        self.condition = Some(value);
        self
    }

    pub fn evaluation_window(
        mut self,
        value: crate::datadogV2::model::SecurityMonitoringRuleEvaluationWindow,
    ) -> Self {
        self.evaluation_window = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
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

impl Default for SecurityMonitoringRuleSequenceDetectionStep {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SecurityMonitoringRuleSequenceDetectionStep {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringRuleSequenceDetectionStepVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringRuleSequenceDetectionStepVisitor {
            type Value = SecurityMonitoringRuleSequenceDetectionStep;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut condition: Option<String> = None;
                let mut evaluation_window: Option<
                    crate::datadogV2::model::SecurityMonitoringRuleEvaluationWindow,
                > = None;
                let mut name: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "condition" => {
                            if v.is_null() {
                                continue;
                            }
                            condition = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "evaluationWindow" => {
                            if v.is_null() {
                                continue;
                            }
                            evaluation_window =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _evaluation_window) = evaluation_window {
                                match _evaluation_window {
                                    crate::datadogV2::model::SecurityMonitoringRuleEvaluationWindow::UnparsedObject(_evaluation_window) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SecurityMonitoringRuleSequenceDetectionStep {
                    condition,
                    evaluation_window,
                    name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringRuleSequenceDetectionStepVisitor)
    }
}

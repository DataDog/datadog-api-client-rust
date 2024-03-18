// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Options on rules.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringRuleOptions {
    /// Options for cloud_configuration rules.
    /// Fields `resourceType` and `regoRule` are mandatory when managing custom `cloud_configuration` rules.
    ///
    #[serde(rename = "complianceRuleOptions")]
    pub compliance_rule_options:
        Option<crate::datadogV2::model::CloudConfigurationComplianceRuleOptions>,
    /// If true, signals in non-production environments have a lower severity than what is defined by the rule case, which can reduce signal noise.
    /// The severity is decreased by one level: `CRITICAL` in production becomes `HIGH` in non-production, `HIGH` becomes `MEDIUM` and so on. `INFO` remains `INFO`.
    /// The decrement is applied when the environment tag of the signal starts with `staging`, `test` or `dev`.
    #[serde(rename = "decreaseCriticalityBasedOnEnv")]
    pub decrease_criticality_based_on_env: Option<bool>,
    /// The detection method.
    #[serde(rename = "detectionMethod")]
    pub detection_method: Option<crate::datadogV2::model::SecurityMonitoringRuleDetectionMethod>,
    /// A time window is specified to match when at least one of the cases matches true. This is a sliding window
    /// and evaluates in real time.
    #[serde(rename = "evaluationWindow")]
    pub evaluation_window: Option<crate::datadogV2::model::SecurityMonitoringRuleEvaluationWindow>,
    /// Hardcoded evaluator type.
    #[serde(rename = "hardcodedEvaluatorType")]
    pub hardcoded_evaluator_type:
        Option<crate::datadogV2::model::SecurityMonitoringRuleHardcodedEvaluatorType>,
    /// Options on impossible travel rules.
    #[serde(rename = "impossibleTravelOptions")]
    pub impossible_travel_options:
        Option<crate::datadogV2::model::SecurityMonitoringRuleImpossibleTravelOptions>,
    /// Once a signal is generated, the signal will remain “open” if a case is matched at least once within
    /// this keep alive window.
    #[serde(rename = "keepAlive")]
    pub keep_alive: Option<crate::datadogV2::model::SecurityMonitoringRuleKeepAlive>,
    /// A signal will “close” regardless of the query being matched once the time exceeds the maximum duration.
    /// This time is calculated from the first seen timestamp.
    #[serde(rename = "maxSignalDuration")]
    pub max_signal_duration:
        Option<crate::datadogV2::model::SecurityMonitoringRuleMaxSignalDuration>,
    /// Options on new value rules.
    #[serde(rename = "newValueOptions")]
    pub new_value_options: Option<crate::datadogV2::model::SecurityMonitoringRuleNewValueOptions>,
    /// Options on third party rules.
    #[serde(rename = "thirdPartyRuleOptions")]
    pub third_party_rule_options:
        Option<crate::datadogV2::model::SecurityMonitoringRuleThirdPartyOptions>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringRuleOptions {
    pub fn new() -> SecurityMonitoringRuleOptions {
        SecurityMonitoringRuleOptions {
            compliance_rule_options: None,
            decrease_criticality_based_on_env: None,
            detection_method: None,
            evaluation_window: None,
            hardcoded_evaluator_type: None,
            impossible_travel_options: None,
            keep_alive: None,
            max_signal_duration: None,
            new_value_options: None,
            third_party_rule_options: None,
            _unparsed: false,
        }
    }

    pub fn compliance_rule_options(
        mut self,
        value: crate::datadogV2::model::CloudConfigurationComplianceRuleOptions,
    ) -> Self {
        self.compliance_rule_options = Some(value);
        self
    }

    pub fn decrease_criticality_based_on_env(mut self, value: bool) -> Self {
        self.decrease_criticality_based_on_env = Some(value);
        self
    }

    pub fn detection_method(
        mut self,
        value: crate::datadogV2::model::SecurityMonitoringRuleDetectionMethod,
    ) -> Self {
        self.detection_method = Some(value);
        self
    }

    pub fn evaluation_window(
        mut self,
        value: crate::datadogV2::model::SecurityMonitoringRuleEvaluationWindow,
    ) -> Self {
        self.evaluation_window = Some(value);
        self
    }

    pub fn hardcoded_evaluator_type(
        mut self,
        value: crate::datadogV2::model::SecurityMonitoringRuleHardcodedEvaluatorType,
    ) -> Self {
        self.hardcoded_evaluator_type = Some(value);
        self
    }

    pub fn impossible_travel_options(
        mut self,
        value: crate::datadogV2::model::SecurityMonitoringRuleImpossibleTravelOptions,
    ) -> Self {
        self.impossible_travel_options = Some(value);
        self
    }

    pub fn keep_alive(
        mut self,
        value: crate::datadogV2::model::SecurityMonitoringRuleKeepAlive,
    ) -> Self {
        self.keep_alive = Some(value);
        self
    }

    pub fn max_signal_duration(
        mut self,
        value: crate::datadogV2::model::SecurityMonitoringRuleMaxSignalDuration,
    ) -> Self {
        self.max_signal_duration = Some(value);
        self
    }

    pub fn new_value_options(
        mut self,
        value: crate::datadogV2::model::SecurityMonitoringRuleNewValueOptions,
    ) -> Self {
        self.new_value_options = Some(value);
        self
    }

    pub fn third_party_rule_options(
        mut self,
        value: crate::datadogV2::model::SecurityMonitoringRuleThirdPartyOptions,
    ) -> Self {
        self.third_party_rule_options = Some(value);
        self
    }
}

impl Default for SecurityMonitoringRuleOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SecurityMonitoringRuleOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringRuleOptionsVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringRuleOptionsVisitor {
            type Value = SecurityMonitoringRuleOptions;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut compliance_rule_options: Option<
                    crate::datadogV2::model::CloudConfigurationComplianceRuleOptions,
                > = None;
                let mut decrease_criticality_based_on_env: Option<bool> = None;
                let mut detection_method: Option<
                    crate::datadogV2::model::SecurityMonitoringRuleDetectionMethod,
                > = None;
                let mut evaluation_window: Option<
                    crate::datadogV2::model::SecurityMonitoringRuleEvaluationWindow,
                > = None;
                let mut hardcoded_evaluator_type: Option<
                    crate::datadogV2::model::SecurityMonitoringRuleHardcodedEvaluatorType,
                > = None;
                let mut impossible_travel_options: Option<
                    crate::datadogV2::model::SecurityMonitoringRuleImpossibleTravelOptions,
                > = None;
                let mut keep_alive: Option<
                    crate::datadogV2::model::SecurityMonitoringRuleKeepAlive,
                > = None;
                let mut max_signal_duration: Option<
                    crate::datadogV2::model::SecurityMonitoringRuleMaxSignalDuration,
                > = None;
                let mut new_value_options: Option<
                    crate::datadogV2::model::SecurityMonitoringRuleNewValueOptions,
                > = None;
                let mut third_party_rule_options: Option<
                    crate::datadogV2::model::SecurityMonitoringRuleThirdPartyOptions,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "complianceRuleOptions" => {
                            if v.is_null() {
                                continue;
                            }
                            compliance_rule_options =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "decreaseCriticalityBasedOnEnv" => {
                            if v.is_null() {
                                continue;
                            }
                            decrease_criticality_based_on_env =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "detectionMethod" => {
                            if v.is_null() {
                                continue;
                            }
                            detection_method =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _detection_method) = detection_method {
                                match _detection_method {
                                    crate::datadogV2::model::SecurityMonitoringRuleDetectionMethod::UnparsedObject(_detection_method) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
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
                        "hardcodedEvaluatorType" => {
                            if v.is_null() {
                                continue;
                            }
                            hardcoded_evaluator_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _hardcoded_evaluator_type) = hardcoded_evaluator_type {
                                match _hardcoded_evaluator_type {
                                    crate::datadogV2::model::SecurityMonitoringRuleHardcodedEvaluatorType::UnparsedObject(_hardcoded_evaluator_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "impossibleTravelOptions" => {
                            if v.is_null() {
                                continue;
                            }
                            impossible_travel_options =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "keepAlive" => {
                            if v.is_null() {
                                continue;
                            }
                            keep_alive = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _keep_alive) = keep_alive {
                                match _keep_alive {
                                    crate::datadogV2::model::SecurityMonitoringRuleKeepAlive::UnparsedObject(_keep_alive) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "maxSignalDuration" => {
                            if v.is_null() {
                                continue;
                            }
                            max_signal_duration =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _max_signal_duration) = max_signal_duration {
                                match _max_signal_duration {
                                    crate::datadogV2::model::SecurityMonitoringRuleMaxSignalDuration::UnparsedObject(_max_signal_duration) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "newValueOptions" => {
                            if v.is_null() {
                                continue;
                            }
                            new_value_options =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "thirdPartyRuleOptions" => {
                            if v.is_null() {
                                continue;
                            }
                            third_party_rule_options =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SecurityMonitoringRuleOptions {
                    compliance_rule_options,
                    decrease_criticality_based_on_env,
                    detection_method,
                    evaluation_window,
                    hardcoded_evaluator_type,
                    impossible_travel_options,
                    keep_alive,
                    max_signal_duration,
                    new_value_options,
                    third_party_rule_options,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringRuleOptionsVisitor)
    }
}

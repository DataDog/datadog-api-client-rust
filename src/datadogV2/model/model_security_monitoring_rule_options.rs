// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Options on rules.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityMonitoringRuleOptions {
    /// Options for cloud_configuration rules.
    /// Fields `resourceType` and `regoRule` are mandatory when managing custom `cloud_configuration` rules.
    ///
    #[serde(rename = "complianceRuleOptions")]
    pub compliance_rule_options:
        Option<Box<crate::datadogV2::model::CloudConfigurationComplianceRuleOptions>>,
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
        Option<Box<crate::datadogV2::model::SecurityMonitoringRuleImpossibleTravelOptions>>,
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
    pub new_value_options:
        Option<Box<crate::datadogV2::model::SecurityMonitoringRuleNewValueOptions>>,
    /// Options on third party rules.
    #[serde(rename = "thirdPartyRuleOptions")]
    pub third_party_rule_options:
        Option<Box<crate::datadogV2::model::SecurityMonitoringRuleThirdPartyOptions>>,
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
        }
    }
}
impl Default for SecurityMonitoringRuleOptions {
    fn default() -> Self {
        Self::new()
    }
}

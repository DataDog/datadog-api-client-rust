// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityMonitoringRuleOptions {
    /// Options for cloud_configuration rules.
Fields `resourceType` and `regoRule` are mandatory when managing custom `cloud_configuration` rules.

    #[serde(rename = "complianceRuleOptions", skip_serializing_if = "Option::is_none")]
    pub compliance_rule_options: CloudConfigurationComplianceRuleOptions,
    /// If true, signals in non-production environments have a lower severity than what is defined by the rule case, which can reduce signal noise.
The severity is decreased by one level: `CRITICAL` in production becomes `HIGH` in non-production, `HIGH` becomes `MEDIUM` and so on. `INFO` remains `INFO`.
The decrement is applied when the environment tag of the signal starts with `staging`, `test` or `dev`.
    #[serde(rename = "decreaseCriticalityBasedOnEnv", skip_serializing_if = "Option::is_none")]
    pub decrease_criticality_based_on_env: bool,
    /// The detection method.
    #[serde(rename = "detectionMethod", skip_serializing_if = "Option::is_none")]
    pub detection_method: SecurityMonitoringRuleDetectionMethod,
    /// A time window is specified to match when at least one of the cases matches true. This is a sliding window
and evaluates in real time.
    #[serde(rename = "evaluationWindow", skip_serializing_if = "Option::is_none")]
    pub evaluation_window: SecurityMonitoringRuleEvaluationWindow,
    /// Hardcoded evaluator type.
    #[serde(rename = "hardcodedEvaluatorType", skip_serializing_if = "Option::is_none")]
    pub hardcoded_evaluator_type: SecurityMonitoringRuleHardcodedEvaluatorType,
    /// Options on impossible travel rules.
    #[serde(rename = "impossibleTravelOptions", skip_serializing_if = "Option::is_none")]
    pub impossible_travel_options: SecurityMonitoringRuleImpossibleTravelOptions,
    /// Once a signal is generated, the signal will remain “open” if a case is matched at least once within
this keep alive window.
    #[serde(rename = "keepAlive", skip_serializing_if = "Option::is_none")]
    pub keep_alive: SecurityMonitoringRuleKeepAlive,
    /// A signal will “close” regardless of the query being matched once the time exceeds the maximum duration.
This time is calculated from the first seen timestamp.
    #[serde(rename = "maxSignalDuration", skip_serializing_if = "Option::is_none")]
    pub max_signal_duration: SecurityMonitoringRuleMaxSignalDuration,
    /// Options on new value rules.
    #[serde(rename = "newValueOptions", skip_serializing_if = "Option::is_none")]
    pub new_value_options: SecurityMonitoringRuleNewValueOptions,
}


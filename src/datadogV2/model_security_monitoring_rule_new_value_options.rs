// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityMonitoringRuleNewValueOptions {
    /// The duration in days after which a learned value is forgotten.
    #[serde(rename = "forgetAfter", skip_serializing_if = "Option::is_none")]
    pub forget_after: SecurityMonitoringRuleNewValueOptionsForgetAfter,
    /// The duration in days during which values are learned, and after which signals will be generated for values that
weren't learned. If set to 0, a signal will be generated for all new values after the first value is learned.
    #[serde(rename = "learningDuration", skip_serializing_if = "Option::is_none")]
    pub learning_duration: SecurityMonitoringRuleNewValueOptionsLearningDuration,
    /// The learning method used to determine when signals should be generated for values that weren't learned.
    #[serde(rename = "learningMethod", skip_serializing_if = "Option::is_none")]
    pub learning_method: SecurityMonitoringRuleNewValueOptionsLearningMethod,
    /// A number of occurrences after which signals will be generated for values that weren't learned.
    #[serde(rename = "learningThreshold", skip_serializing_if = "Option::is_none")]
    pub learning_threshold: SecurityMonitoringRuleNewValueOptionsLearningThreshold,
}


// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Options on new value rules.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityMonitoringRuleNewValueOptions {
    /// The duration in days after which a learned value is forgotten.
    #[serde(rename = "forgetAfter")]
    pub forget_after:
        Option<crate::datadogV2::model::SecurityMonitoringRuleNewValueOptionsForgetAfter>,
    /// The duration in days during which values are learned, and after which signals will be generated for values that
    /// weren't learned. If set to 0, a signal will be generated for all new values after the first value is learned.
    #[serde(rename = "learningDuration")]
    pub learning_duration:
        Option<crate::datadogV2::model::SecurityMonitoringRuleNewValueOptionsLearningDuration>,
    /// The learning method used to determine when signals should be generated for values that weren't learned.
    #[serde(rename = "learningMethod")]
    pub learning_method:
        Option<crate::datadogV2::model::SecurityMonitoringRuleNewValueOptionsLearningMethod>,
    /// A number of occurrences after which signals will be generated for values that weren't learned.
    #[serde(rename = "learningThreshold")]
    pub learning_threshold:
        Option<crate::datadogV2::model::SecurityMonitoringRuleNewValueOptionsLearningThreshold>,
}

impl SecurityMonitoringRuleNewValueOptions {
    pub fn new() -> SecurityMonitoringRuleNewValueOptions {
        SecurityMonitoringRuleNewValueOptions {
            forget_after: None,
            learning_duration: None,
            learning_method: None,
            learning_threshold: None,
        }
    }

    pub fn forget_after(
        mut self,
        value: crate::datadogV2::model::SecurityMonitoringRuleNewValueOptionsForgetAfter,
    ) -> Self {
        self.forget_after = Some(value);
        self
    }

    pub fn learning_duration(
        mut self,
        value: crate::datadogV2::model::SecurityMonitoringRuleNewValueOptionsLearningDuration,
    ) -> Self {
        self.learning_duration = Some(value);
        self
    }

    pub fn learning_method(
        mut self,
        value: crate::datadogV2::model::SecurityMonitoringRuleNewValueOptionsLearningMethod,
    ) -> Self {
        self.learning_method = Some(value);
        self
    }

    pub fn learning_threshold(
        mut self,
        value: crate::datadogV2::model::SecurityMonitoringRuleNewValueOptionsLearningThreshold,
    ) -> Self {
        self.learning_threshold = Some(value);
        self
    }
}

impl Default for SecurityMonitoringRuleNewValueOptions {
    fn default() -> Self {
        Self::new()
    }
}

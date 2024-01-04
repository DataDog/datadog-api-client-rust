// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Create a new cloud configuration rule.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudConfigurationRuleCreatePayload {
    /// Description of generated findings and signals (severity and channels to be notified in case of a signal). Must contain exactly one item.
    ///
    #[serde(rename = "cases")]
    pub cases: Vec<crate::datadogV2::model::CloudConfigurationRuleCaseCreate>,
    /// How to generate compliance signals. Useful for cloud_configuration rules only.
    #[serde(rename = "complianceSignalOptions")]
    pub compliance_signal_options:
        Box<crate::datadogV2::model::CloudConfigurationRuleComplianceSignalOptions>,
    /// Additional queries to filter matched events before they are processed.
    #[serde(rename = "filters")]
    pub filters: Option<Vec<crate::datadogV2::model::SecurityMonitoringFilter>>,
    /// Whether the rule is enabled.
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
    /// Message in markdown format for generated findings and signals.
    #[serde(rename = "message")]
    pub message: String,
    /// The name of the rule.
    #[serde(rename = "name")]
    pub name: String,
    /// Options on cloud configuration rules.
    #[serde(rename = "options")]
    pub options: Box<crate::datadogV2::model::CloudConfigurationRuleOptions>,
    /// Tags for generated findings and signals.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// The rule type.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::CloudConfigurationRuleType>,
}

impl CloudConfigurationRuleCreatePayload {
    pub fn new(
        cases: Vec<crate::datadogV2::model::CloudConfigurationRuleCaseCreate>,
        compliance_signal_options: Box<
            crate::datadogV2::model::CloudConfigurationRuleComplianceSignalOptions,
        >,
        is_enabled: bool,
        message: String,
        name: String,
        options: Box<crate::datadogV2::model::CloudConfigurationRuleOptions>,
    ) -> CloudConfigurationRuleCreatePayload {
        CloudConfigurationRuleCreatePayload {
            cases,
            compliance_signal_options,
            filters: None,
            is_enabled,
            message,
            name,
            options,
            tags: None,
            type_: None,
        }
    }
}
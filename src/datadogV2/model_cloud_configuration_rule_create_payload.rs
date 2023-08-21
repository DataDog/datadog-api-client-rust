// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudConfigurationRuleCreatePayload {
    /// Description of generated findings and signals (severity and channels to be notified in case of a signal). Must contain exactly one item.

    #[serde(rename = "cases", skip_serializing_if = "Option::is_none")]
    pub cases: Vec<CloudConfigurationRuleCaseCreate>,
    /// How to generate compliance signals. Useful for cloud_configuration rules only.
    #[serde(rename = "complianceSignalOptions", skip_serializing_if = "Option::is_none")]
    pub compliance_signal_options: CloudConfigurationRuleComplianceSignalOptions,
    /// Additional queries to filter matched events before they are processed.
    #[serde(rename = "filters", skip_serializing_if = "Option::is_none")]
    pub filters: Vec<SecurityMonitoringFilter>,
    /// Whether the rule is enabled.
    #[serde(rename = "isEnabled", skip_serializing_if = "Option::is_none")]
    pub is_enabled: bool,
    /// Message in markdown format for generated findings and signals.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: String,
    /// The name of the rule.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// Options on cloud configuration rules.
    #[serde(rename = "options")]
    pub options: CloudConfigurationRuleOptions,
    /// Tags for generated findings and signals.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Vec<String>,
    /// The rule type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: CloudConfigurationRuleType,
}


// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityMonitoringRuleUpdatePayload {
    /// Cases for generating signals.
    #[serde(rename = "cases", skip_serializing_if = "Option::is_none")]
    pub cases: Vec<SecurityMonitoringRuleCase>,
    /// How to generate compliance signals. Useful for cloud_configuration rules only.
    #[serde(rename = "complianceSignalOptions", skip_serializing_if = "Option::is_none")]
    pub compliance_signal_options: CloudConfigurationRuleComplianceSignalOptions,
    /// Additional queries to filter matched events before they are processed.
    #[serde(rename = "filters", skip_serializing_if = "Option::is_none")]
    pub filters: Vec<SecurityMonitoringFilter>,
    /// Whether the notifications include the triggering group-by values in their title.
    #[serde(rename = "hasExtendedTitle", skip_serializing_if = "Option::is_none")]
    pub has_extended_title: bool,
    /// Whether the rule is enabled.
    #[serde(rename = "isEnabled", skip_serializing_if = "Option::is_none")]
    pub is_enabled: bool,
    /// Message for generated signals.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: String,
    /// Name of the rule.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// Options on rules.
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: SecurityMonitoringRuleOptions,
    /// Queries for selecting logs which are part of the rule.
    #[serde(rename = "queries", skip_serializing_if = "Option::is_none")]
    pub queries: Vec<SecurityMonitoringRuleQuery>,
    /// Tags for generated signals.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Vec<String>,
    /// The version of the rule being updated.
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: i32,
}


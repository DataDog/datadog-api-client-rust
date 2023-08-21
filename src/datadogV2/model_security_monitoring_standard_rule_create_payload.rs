// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityMonitoringStandardRuleCreatePayload {
    /// Cases for generating signals.
    #[serde(rename = "cases", skip_serializing_if = "Option::is_none")]
    pub cases: Vec<SecurityMonitoringRuleCaseCreate>,
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
    /// The name of the rule.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// Options on rules.
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: SecurityMonitoringRuleOptions,
    /// Queries for selecting logs which are part of the rule.
    #[serde(rename = "queries", skip_serializing_if = "Option::is_none")]
    pub queries: Vec<SecurityMonitoringStandardRuleQuery>,
    /// Tags for generated signals.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Vec<String>,
    /// The rule type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: SecurityMonitoringRuleTypeCreate,
}


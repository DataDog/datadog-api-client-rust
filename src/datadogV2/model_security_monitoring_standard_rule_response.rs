// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityMonitoringStandardRuleResponse {
    /// Cases for generating signals.
    #[serde(rename = "cases", skip_serializing_if = "Option::is_none")]
    pub cases: Vec<SecurityMonitoringRuleCase>,
    /// How to generate compliance signals. Useful for cloud_configuration rules only.
    #[serde(rename = "complianceSignalOptions", skip_serializing_if = "Option::is_none")]
    pub compliance_signal_options: CloudConfigurationRuleComplianceSignalOptions,
    /// When the rule was created, timestamp in milliseconds.
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: i64,
    /// User ID of the user who created the rule.
    #[serde(rename = "creationAuthorId", skip_serializing_if = "Option::is_none")]
    pub creation_author_id: i64,
    /// When the rule will be deprecated, timestamp in milliseconds.
    #[serde(rename = "deprecationDate", skip_serializing_if = "Option::is_none")]
    pub deprecation_date: i64,
    /// Additional queries to filter matched events before they are processed.
    #[serde(rename = "filters", skip_serializing_if = "Option::is_none")]
    pub filters: Vec<SecurityMonitoringFilter>,
    /// Whether the notifications include the triggering group-by values in their title.
    #[serde(rename = "hasExtendedTitle", skip_serializing_if = "Option::is_none")]
    pub has_extended_title: bool,
    /// The ID of the rule.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: String,
    /// Whether the rule is included by default.
    #[serde(rename = "isDefault", skip_serializing_if = "Option::is_none")]
    pub is_default: bool,
    /// Whether the rule has been deleted.
    #[serde(rename = "isDeleted", skip_serializing_if = "Option::is_none")]
    pub is_deleted: bool,
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
    pub type_: SecurityMonitoringRuleTypeRead,
    /// User ID of the user who updated the rule.
    #[serde(rename = "updateAuthorId", skip_serializing_if = "Option::is_none")]
    pub update_author_id: i64,
    /// The version of the rule.
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: i64,
}


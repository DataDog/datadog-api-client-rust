// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Create a new rule.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityMonitoringStandardRuleCreatePayload {
    /// Cases for generating signals.
    #[serde(rename = "cases")]
    pub cases: Vec<crate::datadogV2::model::SecurityMonitoringRuleCaseCreate>,
    /// Additional queries to filter matched events before they are processed.
    #[serde(rename = "filters")]
    pub filters: Option<Vec<crate::datadogV2::model::SecurityMonitoringFilter>>,
    /// Whether the notifications include the triggering group-by values in their title.
    #[serde(rename = "hasExtendedTitle")]
    pub has_extended_title: Option<bool>,
    /// Whether the rule is enabled.
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
    /// Message for generated signals.
    #[serde(rename = "message")]
    pub message: String,
    /// The name of the rule.
    #[serde(rename = "name")]
    pub name: String,
    /// Options on rules.
    #[serde(rename = "options")]
    pub options: Box<crate::datadogV2::model::SecurityMonitoringRuleOptions>,
    /// Queries for selecting logs which are part of the rule.
    #[serde(rename = "queries")]
    pub queries: Vec<crate::datadogV2::model::SecurityMonitoringStandardRuleQuery>,
    /// Tags for generated signals.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// Cases for generating signals from third party rules. Only available for third party rules.
    #[serde(rename = "thirdPartyCases")]
    pub third_party_cases:
        Option<Vec<crate::datadogV2::model::SecurityMonitoringThirdPartyRuleCaseCreate>>,
    /// The rule type.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::SecurityMonitoringRuleTypeCreate>,
}

impl SecurityMonitoringStandardRuleCreatePayload {
    pub fn new(
        cases: Vec<crate::datadogV2::model::SecurityMonitoringRuleCaseCreate>,
        is_enabled: bool,
        message: String,
        name: String,
        options: Box<crate::datadogV2::model::SecurityMonitoringRuleOptions>,
        queries: Vec<crate::datadogV2::model::SecurityMonitoringStandardRuleQuery>,
    ) -> SecurityMonitoringStandardRuleCreatePayload {
        SecurityMonitoringStandardRuleCreatePayload {
            cases,
            filters: None,
            has_extended_title: None,
            is_enabled,
            message,
            name,
            options,
            queries,
            tags: None,
            third_party_cases: None,
            type_: None,
        }
    }
}

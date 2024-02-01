// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Update an existing rule.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityMonitoringRuleUpdatePayload {
    /// Cases for generating signals.
    #[serde(rename = "cases")]
    pub cases: Option<Vec<crate::datadogV2::model::SecurityMonitoringRuleCase>>,
    /// How to generate compliance signals. Useful for cloud_configuration rules only.
    #[serde(rename = "complianceSignalOptions")]
    pub compliance_signal_options:
        Option<crate::datadogV2::model::CloudConfigurationRuleComplianceSignalOptions>,
    /// Additional queries to filter matched events before they are processed.
    #[serde(rename = "filters")]
    pub filters: Option<Vec<crate::datadogV2::model::SecurityMonitoringFilter>>,
    /// Whether the notifications include the triggering group-by values in their title.
    #[serde(rename = "hasExtendedTitle")]
    pub has_extended_title: Option<bool>,
    /// Whether the rule is enabled.
    #[serde(rename = "isEnabled")]
    pub is_enabled: Option<bool>,
    /// Message for generated signals.
    #[serde(rename = "message")]
    pub message: Option<String>,
    /// Name of the rule.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Options on rules.
    #[serde(rename = "options")]
    pub options: Option<crate::datadogV2::model::SecurityMonitoringRuleOptions>,
    /// Queries for selecting logs which are part of the rule.
    #[serde(rename = "queries")]
    pub queries: Option<Vec<crate::datadogV2::model::SecurityMonitoringRuleQuery>>,
    /// Tags for generated signals.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// Cases for generating signals from third party rules. Only available for third party rules.
    #[serde(rename = "thirdPartyCases")]
    pub third_party_cases:
        Option<Vec<crate::datadogV2::model::SecurityMonitoringThirdPartyRuleCase>>,
    /// The version of the rule being updated.
    #[serde(rename = "version")]
    pub version: Option<i32>,
}

impl SecurityMonitoringRuleUpdatePayload {
    pub fn new() -> SecurityMonitoringRuleUpdatePayload {
        SecurityMonitoringRuleUpdatePayload {
            cases: None,
            compliance_signal_options: None,
            filters: None,
            has_extended_title: None,
            is_enabled: None,
            message: None,
            name: None,
            options: None,
            queries: None,
            tags: None,
            third_party_cases: None,
            version: None,
        }
    }

    pub fn cases(
        &mut self,
        value: Vec<crate::datadogV2::model::SecurityMonitoringRuleCase>,
    ) -> &mut Self {
        self.cases = Some(value);
        self
    }

    pub fn compliance_signal_options(
        &mut self,
        value: crate::datadogV2::model::CloudConfigurationRuleComplianceSignalOptions,
    ) -> &mut Self {
        self.compliance_signal_options = Some(value);
        self
    }

    pub fn filters(
        &mut self,
        value: Vec<crate::datadogV2::model::SecurityMonitoringFilter>,
    ) -> &mut Self {
        self.filters = Some(value);
        self
    }

    pub fn has_extended_title(&mut self, value: bool) -> &mut Self {
        self.has_extended_title = Some(value);
        self
    }

    pub fn is_enabled(&mut self, value: bool) -> &mut Self {
        self.is_enabled = Some(value);
        self
    }

    pub fn message(&mut self, value: String) -> &mut Self {
        self.message = Some(value);
        self
    }

    pub fn name(&mut self, value: String) -> &mut Self {
        self.name = Some(value);
        self
    }

    pub fn options(
        &mut self,
        value: crate::datadogV2::model::SecurityMonitoringRuleOptions,
    ) -> &mut Self {
        self.options = Some(value);
        self
    }

    pub fn queries(
        &mut self,
        value: Vec<crate::datadogV2::model::SecurityMonitoringRuleQuery>,
    ) -> &mut Self {
        self.queries = Some(value);
        self
    }

    pub fn tags(&mut self, value: Vec<String>) -> &mut Self {
        self.tags = Some(value);
        self
    }

    pub fn third_party_cases(
        &mut self,
        value: Vec<crate::datadogV2::model::SecurityMonitoringThirdPartyRuleCase>,
    ) -> &mut Self {
        self.third_party_cases = Some(value);
        self
    }

    pub fn version(&mut self, value: i32) -> &mut Self {
        self.version = Some(value);
        self
    }
}

impl Default for SecurityMonitoringRuleUpdatePayload {
    fn default() -> Self {
        Self::new()
    }
}

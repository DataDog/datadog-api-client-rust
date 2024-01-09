// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Options for cloud_configuration rules.
/// Fields `resourceType` and `regoRule` are mandatory when managing custom `cloud_configuration` rules.
///
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudConfigurationComplianceRuleOptions {
    /// Whether the rule is a complex one.
    /// Must be set to true if `regoRule.resourceTypes` contains more than one item. Defaults to false.
    ///
    #[serde(rename = "complexRule")]
    pub complex_rule: Option<bool>,
    /// Rule details.
    #[serde(rename = "regoRule")]
    pub rego_rule: Option<Box<crate::datadogV2::model::CloudConfigurationRegoRule>>,
    /// Main resource type to be checked by the rule. It should be specified again in `regoRule.resourceTypes`.
    ///
    #[serde(rename = "resourceType")]
    pub resource_type: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}

impl CloudConfigurationComplianceRuleOptions {
    pub fn new() -> CloudConfigurationComplianceRuleOptions {
        CloudConfigurationComplianceRuleOptions {
            complex_rule: None,
            rego_rule: None,
            resource_type: None,
            additional_properties: std::collections::BTreeMap::new(),
        }
    }
}
impl Default for CloudConfigurationComplianceRuleOptions {
    fn default() -> Self {
        Self::new()
    }
}

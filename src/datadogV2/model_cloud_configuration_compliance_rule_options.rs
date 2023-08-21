// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudConfigurationComplianceRuleOptions {
    /// Whether the rule is a complex one.
Must be set to true if `regoRule.resourceTypes` contains more than one item. Defaults to false.

    #[serde(rename = "complexRule", skip_serializing_if = "Option::is_none")]
    pub complex_rule: bool,
    /// Rule details.
    #[serde(rename = "regoRule")]
    pub rego_rule: CloudConfigurationRegoRule,
    /// Main resource type to be checked by the rule. It should be specified again in `regoRule.resourceTypes`.

    #[serde(rename = "resourceType", skip_serializing_if = "Option::is_none")]
    pub resource_type: String,
}


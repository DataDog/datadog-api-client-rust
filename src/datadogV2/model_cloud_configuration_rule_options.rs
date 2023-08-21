// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudConfigurationRuleOptions {
    /// Options for cloud_configuration rules.
Fields `resourceType` and `regoRule` are mandatory when managing custom `cloud_configuration` rules.

    #[serde(rename = "complianceRuleOptions", skip_serializing_if = "Option::is_none")]
    pub compliance_rule_options: CloudConfigurationComplianceRuleOptions,
}


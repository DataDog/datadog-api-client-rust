// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Options on cloud configuration rules.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudConfigurationRuleOptions {
    /// Options for cloud_configuration rules.
    /// Fields `resourceType` and `regoRule` are mandatory when managing custom `cloud_configuration` rules.
    ///
    #[serde(rename = "complianceRuleOptions")]
    pub compliance_rule_options: crate::datadogV2::model::CloudConfigurationComplianceRuleOptions,
}

impl CloudConfigurationRuleOptions {
    pub fn new(
        compliance_rule_options: crate::datadogV2::model::CloudConfigurationComplianceRuleOptions,
    ) -> CloudConfigurationRuleOptions {
        CloudConfigurationRuleOptions {
            compliance_rule_options,
        }
    }
}

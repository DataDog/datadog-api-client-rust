// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SensitiveDataScannerGroupRelationships {
    /// A Sensitive Data Scanner configuration data.
    #[serde(rename = "configuration", skip_serializing_if = "Option::is_none")]
    pub configuration: SensitiveDataScannerConfigurationData,
    /// Rules included in the group.
    #[serde(rename = "rules", skip_serializing_if = "Option::is_none")]
    pub rules: SensitiveDataScannerRuleData,
}


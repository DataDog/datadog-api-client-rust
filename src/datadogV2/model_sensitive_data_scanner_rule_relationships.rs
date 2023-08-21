// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SensitiveDataScannerRuleRelationships {
    /// A scanning group data.
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: SensitiveDataScannerGroupData,
    /// A standard pattern.
    #[serde(rename = "standard_pattern", skip_serializing_if = "Option::is_none")]
    pub standard_pattern: SensitiveDataScannerStandardPatternData,
}


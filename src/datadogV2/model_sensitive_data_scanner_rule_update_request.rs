// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SensitiveDataScannerRuleUpdateRequest {
    /// Data related to the update of a rule.
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: SensitiveDataScannerRuleUpdate,
    /// Meta payload containing information about the API.
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: SensitiveDataScannerMetaVersionOnly,
}


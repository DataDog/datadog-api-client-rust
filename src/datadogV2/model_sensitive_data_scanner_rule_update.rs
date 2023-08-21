// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SensitiveDataScannerRuleUpdate {
    /// Attributes of the Sensitive Data Scanner rule.
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: SensitiveDataScannerRuleAttributes,
    /// ID of the rule.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: String,
    /// Relationships of a scanning rule.
    #[serde(rename = "relationships", skip_serializing_if = "Option::is_none")]
    pub relationships: SensitiveDataScannerRuleRelationships,
    /// Sensitive Data Scanner rule type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: SensitiveDataScannerRuleType,
}


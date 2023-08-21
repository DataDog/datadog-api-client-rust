// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SensitiveDataScannerGroupCreate {
    /// Attributes of the Sensitive Data Scanner group.
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: SensitiveDataScannerGroupAttributes,
    /// Relationships of the group.
    #[serde(rename = "relationships", skip_serializing_if = "Option::is_none")]
    pub relationships: SensitiveDataScannerGroupRelationships,
    /// Sensitive Data Scanner group type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: SensitiveDataScannerGroupType,
}


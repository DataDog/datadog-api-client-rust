// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SensitiveDataScannerReorderConfig {
    /// ID of the configuration.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: String,
    /// Relationships of the configuration.
    #[serde(rename = "relationships", skip_serializing_if = "Option::is_none")]
    pub relationships: SensitiveDataScannerConfigurationRelationships,
    /// Sensitive Data Scanner configuration type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: SensitiveDataScannerConfigurationType,
}


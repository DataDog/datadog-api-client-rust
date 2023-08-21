// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsArchiveDestinationAzure {
    /// The container where the archive will be stored.
    #[serde(rename = "container", skip_serializing_if = "Option::is_none")]
    pub container: String,
    /// The Azure archive's integration destination.
    #[serde(rename = "integration")]
    pub integration: LogsArchiveIntegrationAzure,
    /// The archive path.
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: String,
    /// The region where the archive will be stored.
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: String,
    /// The associated storage account.
    #[serde(rename = "storage_account", skip_serializing_if = "Option::is_none")]
    pub storage_account: String,
    /// Type of the Azure archive destination.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: LogsArchiveDestinationAzureType,
}


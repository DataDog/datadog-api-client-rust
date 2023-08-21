// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsArchiveDestinationGCS {
    /// The bucket where the archive will be stored.
    #[serde(rename = "bucket", skip_serializing_if = "Option::is_none")]
    pub bucket: String,
    /// The GCS archive's integration destination.
    #[serde(rename = "integration")]
    pub integration: LogsArchiveIntegrationGCS,
    /// The archive path.
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: String,
    /// Type of the GCS archive destination.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: LogsArchiveDestinationGCSType,
}


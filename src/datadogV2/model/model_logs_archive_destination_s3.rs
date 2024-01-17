// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The S3 archive destination.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsArchiveDestinationS3 {
    /// The bucket where the archive will be stored.
    #[serde(rename = "bucket")]
    pub bucket: String,
    /// The S3 Archive's integration destination.
    #[serde(rename = "integration")]
    pub integration: Box<crate::datadogV2::model::LogsArchiveIntegrationS3>,
    /// The archive path.
    #[serde(rename = "path")]
    pub path: Option<String>,
    /// Type of the S3 archive destination.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::LogsArchiveDestinationS3Type,
}

impl LogsArchiveDestinationS3 {
    pub fn new(
        bucket: String,
        integration: Box<crate::datadogV2::model::LogsArchiveIntegrationS3>,
        type_: crate::datadogV2::model::LogsArchiveDestinationS3Type,
    ) -> LogsArchiveDestinationS3 {
        LogsArchiveDestinationS3 {
            bucket,
            integration,
            path: None,
            type_,
        }
    }
}

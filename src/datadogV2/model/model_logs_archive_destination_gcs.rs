// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The GCS archive destination.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsArchiveDestinationGCS {
    /// The bucket where the archive will be stored.
    #[serde(rename = "bucket")]
    pub bucket: String,
    /// The GCS archive's integration destination.
    #[serde(rename = "integration")]
    pub integration: crate::datadogV2::model::LogsArchiveIntegrationGCS,
    /// The archive path.
    #[serde(rename = "path")]
    pub path: Option<String>,
    /// Type of the GCS archive destination.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::LogsArchiveDestinationGCSType,
}

impl LogsArchiveDestinationGCS {
    pub fn new(
        bucket: String,
        integration: crate::datadogV2::model::LogsArchiveIntegrationGCS,
        type_: crate::datadogV2::model::LogsArchiveDestinationGCSType,
    ) -> LogsArchiveDestinationGCS {
        LogsArchiveDestinationGCS {
            bucket,
            integration,
            path: None,
            type_,
        }
    }

    pub fn with_path(&mut self, value: String) -> &mut Self {
        self.path = Some(value);
        self
    }
}

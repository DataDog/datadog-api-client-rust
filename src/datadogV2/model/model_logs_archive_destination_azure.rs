// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The Azure archive destination.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsArchiveDestinationAzure {
    /// The container where the archive will be stored.
    #[serde(rename = "container")]
    pub container: String,
    /// The Azure archive's integration destination.
    #[serde(rename = "integration")]
    pub integration: crate::datadogV2::model::LogsArchiveIntegrationAzure,
    /// The archive path.
    #[serde(rename = "path")]
    pub path: Option<String>,
    /// The region where the archive will be stored.
    #[serde(rename = "region")]
    pub region: Option<String>,
    /// The associated storage account.
    #[serde(rename = "storage_account")]
    pub storage_account: String,
    /// Type of the Azure archive destination.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::LogsArchiveDestinationAzureType,
}

impl LogsArchiveDestinationAzure {
    pub fn new(
        container: String,
        integration: crate::datadogV2::model::LogsArchiveIntegrationAzure,
        storage_account: String,
        type_: crate::datadogV2::model::LogsArchiveDestinationAzureType,
    ) -> LogsArchiveDestinationAzure {
        LogsArchiveDestinationAzure {
            container,
            integration,
            path: None,
            region: None,
            storage_account,
            type_,
        }
    }

    pub fn path(mut self, value: String) -> Self {
        self.path = Some(value);
        self
    }

    pub fn region(mut self, value: String) -> Self {
        self.region = Some(value);
        self
    }
}

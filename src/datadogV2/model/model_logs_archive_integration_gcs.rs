// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The GCS archive's integration destination.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsArchiveIntegrationGCS {
    /// A client email.
    #[serde(rename = "client_email")]
    pub client_email: String,
    /// A project ID.
    #[serde(rename = "project_id")]
    pub project_id: String,
}

impl LogsArchiveIntegrationGCS {
    pub fn new(client_email: String, project_id: String) -> LogsArchiveIntegrationGCS {
        LogsArchiveIntegrationGCS {
            client_email,
            project_id,
        }
    }
}
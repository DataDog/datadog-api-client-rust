// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The Azure archive's integration destination.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsArchiveIntegrationAzure {
    /// A client ID.
    #[serde(rename = "client_id")]
    pub client_id: String,
    /// A tenant ID.
    #[serde(rename = "tenant_id")]
    pub tenant_id: String,
}

impl LogsArchiveIntegrationAzure {
    pub fn new(client_id: String, tenant_id: String) -> LogsArchiveIntegrationAzure {
        LogsArchiveIntegrationAzure {
            client_id,
            tenant_id,
        }
    }
}

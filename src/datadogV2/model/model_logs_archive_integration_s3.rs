// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The S3 Archive's integration destination.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsArchiveIntegrationS3 {
    /// The account ID for the integration.
    #[serde(rename = "account_id")]
    pub account_id: String,
    /// The path of the integration.
    #[serde(rename = "role_name")]
    pub role_name: String,
}

impl LogsArchiveIntegrationS3 {
    pub fn new(account_id: String, role_name: String) -> LogsArchiveIntegrationS3 {
        LogsArchiveIntegrationS3 {
            account_id,
            role_name,
        }
    }
}

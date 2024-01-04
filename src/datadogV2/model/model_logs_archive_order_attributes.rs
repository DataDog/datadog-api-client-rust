// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The attributes associated with the archive order.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsArchiveOrderAttributes {
    /// An ordered array of `<ARCHIVE_ID>` strings, the order of archive IDs in the array
    /// define the overall archives order for Datadog.
    #[serde(rename = "archive_ids")]
    pub archive_ids: Vec<String>,
}

impl LogsArchiveOrderAttributes {
    pub fn new(archive_ids: Vec<String>) -> LogsArchiveOrderAttributes {
        LogsArchiveOrderAttributes { archive_ids }
    }
}

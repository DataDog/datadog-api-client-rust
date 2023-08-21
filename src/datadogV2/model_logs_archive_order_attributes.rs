// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsArchiveOrderAttributes {
    /// An ordered array of `<ARCHIVE_ID>` strings, the order of archive IDs in the array
define the overall archives order for Datadog.
    #[serde(rename = "archive_ids", skip_serializing_if = "Option::is_none")]
    pub archive_ids: Vec<String>,
}


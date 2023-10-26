// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ProcessSummaryAttributes {
    /// Process command line.
    #[serde(rename = "cmdline", skip_serializing_if = "Option::is_none")]
    pub cmdline: Option<String>,
    /// Host running the process.
    #[serde(rename = "host", skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// Process ID.
    #[serde(rename = "pid", skip_serializing_if = "Option::is_none")]
    pub pid: Option<i64>,
    /// Parent process ID.
    #[serde(rename = "ppid", skip_serializing_if = "Option::is_none")]
    pub ppid: Option<i64>,
    /// Time the process was started.
    #[serde(rename = "start", skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    /// List of tags associated with the process.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Time the process was seen.
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    /// Process owner.
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl ProcessSummaryAttributes {
    /// Attributes for a process summary.
    pub fn new() -> ProcessSummaryAttributes {
        ProcessSummaryAttributes {
            cmdline: None,
            host: None,
            pid: None,
            ppid: None,
            start: None,
            tags: None,
            timestamp: None,
            user: None,
        }
    }
}

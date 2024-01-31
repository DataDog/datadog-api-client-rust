// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Attributes for a process summary.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProcessSummaryAttributes {
    /// Process command line.
    #[serde(rename = "cmdline")]
    pub cmdline: Option<String>,
    /// Host running the process.
    #[serde(rename = "host")]
    pub host: Option<String>,
    /// Process ID.
    #[serde(rename = "pid")]
    pub pid: Option<i64>,
    /// Parent process ID.
    #[serde(rename = "ppid")]
    pub ppid: Option<i64>,
    /// Time the process was started.
    #[serde(rename = "start")]
    pub start: Option<String>,
    /// List of tags associated with the process.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// Time the process was seen.
    #[serde(rename = "timestamp")]
    pub timestamp: Option<String>,
    /// Process owner.
    #[serde(rename = "user")]
    pub user: Option<String>,
}

impl ProcessSummaryAttributes {
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

    pub fn with_cmdline(&mut self, value: String) -> &mut Self {
        self.cmdline = Some(value);
        self
    }

    pub fn with_host(&mut self, value: String) -> &mut Self {
        self.host = Some(value);
        self
    }

    pub fn with_pid(&mut self, value: i64) -> &mut Self {
        self.pid = Some(value);
        self
    }

    pub fn with_ppid(&mut self, value: i64) -> &mut Self {
        self.ppid = Some(value);
        self
    }

    pub fn with_start(&mut self, value: String) -> &mut Self {
        self.start = Some(value);
        self
    }

    pub fn with_tags(&mut self, value: Vec<String>) -> &mut Self {
        self.tags = Some(value);
        self
    }

    pub fn with_timestamp(&mut self, value: String) -> &mut Self {
        self.timestamp = Some(value);
        self
    }

    pub fn with_user(&mut self, value: String) -> &mut Self {
        self.user = Some(value);
        self
    }
}
impl Default for ProcessSummaryAttributes {
    fn default() -> Self {
        Self::new()
    }
}

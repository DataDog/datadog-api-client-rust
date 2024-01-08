// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Search and filter query settings.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuditLogsQueryFilter {
    /// Minimum time for the requested events. Supports date, math, and regular timestamps (in milliseconds).
    #[serde(rename = "from")]
    pub from: Option<String>,
    /// Search query following the Audit Logs search syntax.
    #[serde(rename = "query")]
    pub query: Option<String>,
    /// Maximum time for the requested events. Supports date, math, and regular timestamps (in milliseconds).
    #[serde(rename = "to")]
    pub to: Option<String>,
}

impl AuditLogsQueryFilter {
    pub fn new() -> AuditLogsQueryFilter {
        AuditLogsQueryFilter {
            from: None,
            query: None,
            to: None,
        }
    }
}
impl Default for AuditLogsQueryFilter {
    fn default() -> Self {
        Self::new()
    }
}

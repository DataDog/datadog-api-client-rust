// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AuditLogsQueryFilter {
    /// Minimum time for the requested events. Supports date, math, and regular timestamps (in milliseconds).
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    /// Search query following the Audit Logs search syntax.
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Maximum time for the requested events. Supports date, math, and regular timestamps (in milliseconds).
    #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
}

impl AuditLogsQueryFilter {
    /// Search and filter query settings.
    pub fn new() -> AuditLogsQueryFilter {
        AuditLogsQueryFilter {
            from: None,
            query: None,
            to: None,
        }
    }
}

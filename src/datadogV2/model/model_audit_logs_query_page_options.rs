// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Paging attributes for listing events.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AuditLogsQueryPageOptions {
    /// List following results with a cursor provided in the previous query.
    #[serde(rename = "cursor")]
    pub cursor: Option<String>,
    /// Maximum number of events in the response.
    #[serde(rename = "limit")]
    pub limit: Option<i32>,
}

impl AuditLogsQueryPageOptions {
    pub fn new() -> AuditLogsQueryPageOptions {
        AuditLogsQueryPageOptions {
            cursor: None,
            limit: None,
        }
    }
}

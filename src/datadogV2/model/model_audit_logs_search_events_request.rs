// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The request for a Audit Logs events list.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuditLogsSearchEventsRequest {
    /// Search and filter query settings.
    #[serde(rename = "filter")]
    pub filter: Option<Box<crate::datadogV2::model::AuditLogsQueryFilter>>,
    /// Global query options that are used during the query.
    /// Note: Specify either timezone or time offset, not both. Otherwise, the query fails.
    #[serde(rename = "options")]
    pub options: Option<Box<crate::datadogV2::model::AuditLogsQueryOptions>>,
    /// Paging attributes for listing events.
    #[serde(rename = "page")]
    pub page: Option<Box<crate::datadogV2::model::AuditLogsQueryPageOptions>>,
    /// Sort parameters when querying events.
    #[serde(rename = "sort")]
    pub sort: Option<crate::datadogV2::model::AuditLogsSort>,
}

impl AuditLogsSearchEventsRequest {
    pub fn new() -> AuditLogsSearchEventsRequest {
        AuditLogsSearchEventsRequest {
            filter: None,
            options: None,
            page: None,
            sort: None,
        }
    }
}
impl Default for AuditLogsSearchEventsRequest {
    fn default() -> Self {
        Self::new()
    }
}

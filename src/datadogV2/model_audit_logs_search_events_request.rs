// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuditLogsSearchEventsRequest {
    /// Search and filter query settings.
    #[serde(rename = "filter", skip_serializing_if = "Option::is_none")]
    pub filter: AuditLogsQueryFilter,
    /// Global query options that are used during the query.
Note: Specify either timezone or time offset, not both. Otherwise, the query fails.
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: AuditLogsQueryOptions,
    /// Paging attributes for listing events.
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    pub page: AuditLogsQueryPageOptions,
    /// Sort parameters when querying events.
    #[serde(rename = "sort", skip_serializing_if = "Option::is_none")]
    pub sort: AuditLogsSort,
}


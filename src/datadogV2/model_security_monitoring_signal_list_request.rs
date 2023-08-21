// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityMonitoringSignalListRequest {
    /// Search filters for listing security signals.
    #[serde(rename = "filter", skip_serializing_if = "Option::is_none")]
    pub filter: SecurityMonitoringSignalListRequestFilter,
    /// The paging attributes for listing security signals.
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    pub page: SecurityMonitoringSignalListRequestPage,
    /// The sort parameters used for querying security signals.
    #[serde(rename = "sort", skip_serializing_if = "Option::is_none")]
    pub sort: SecurityMonitoringSignalsSort,
}


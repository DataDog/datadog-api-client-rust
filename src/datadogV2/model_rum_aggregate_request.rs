// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RUMAggregateRequest {
    /// The list of metrics or timeseries to compute for the retrieved buckets.
    #[serde(rename = "compute", skip_serializing_if = "Option::is_none")]
    pub compute: Vec<RUMCompute>,
    /// The search and filter query settings.
    #[serde(rename = "filter", skip_serializing_if = "Option::is_none")]
    pub filter: RUMQueryFilter,
    /// The rules for the group by.
    #[serde(rename = "group_by", skip_serializing_if = "Option::is_none")]
    pub group_by: Vec<RUMGroupBy>,
    /// Global query options that are used during the query.
Note: Only supply timezone or time offset, not both. Otherwise, the query fails.
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: RUMQueryOptions,
    /// Paging attributes for listing events.
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    pub page: RUMQueryPageOptions,
}


// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventsListRequest {
    /// The search and filter query settings.
    #[serde(rename = "filter", skip_serializing_if = "Option::is_none")]
    pub filter: EventsQueryFilter,
    /// The global query options that are used. Either provide a timezone or a time offset but not both,
otherwise the query fails.
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: EventsQueryOptions,
    /// Pagination settings.
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    pub page: EventsRequestPage,
    /// The sort parameters when querying events.
    #[serde(rename = "sort", skip_serializing_if = "Option::is_none")]
    pub sort: EventsSort,
}


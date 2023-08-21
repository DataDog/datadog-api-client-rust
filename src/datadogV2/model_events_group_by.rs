// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventsGroupBy {
    /// The facet by which to split groups.
    #[serde(rename = "facet", skip_serializing_if = "Option::is_none")]
    pub facet: String,
    /// The maximum number of groups to return.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: i32,
    /// The dimension by which to sort a query's results.
    #[serde(rename = "sort")]
    pub sort: EventsGroupBySort,
}


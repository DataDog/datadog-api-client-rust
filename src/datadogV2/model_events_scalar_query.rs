// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventsScalarQuery {
    /// The instructions for what to compute for this query.
    #[serde(rename = "compute")]
    pub compute: EventsCompute,
    /// A data source that is powered by the Events Platform.
    #[serde(rename = "data_source", skip_serializing_if = "Option::is_none")]
    pub data_source: EventsDataSource,
    /// The list of facets on which to split results.
    #[serde(rename = "group_by", skip_serializing_if = "Option::is_none")]
    pub group_by: Vec<EventsGroupBy>,
    /// The indexes in which to search.
    #[serde(rename = "indexes", skip_serializing_if = "Option::is_none")]
    pub indexes: Vec<String>,
    /// The variable name for use in formulas.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// Configuration of the search/filter for an events query.
    #[serde(rename = "search", skip_serializing_if = "Option::is_none")]
    pub search: EventsSearch,
}


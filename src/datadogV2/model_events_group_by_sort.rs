// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventsGroupBySort {
    /// The type of aggregation that can be performed on events-based queries.
    #[serde(rename = "aggregation", skip_serializing_if = "Option::is_none")]
    pub aggregation: EventsAggregation,
    /// The metric's calculated value which should be used to define the sort order of a query's results.
    #[serde(rename = "metric", skip_serializing_if = "Option::is_none")]
    pub metric: String,
    /// Direction of sort.
    #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
    pub order: QuerySortOrder,
    /// The type of sort to use on the calculated value.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: EventsSortType,
}


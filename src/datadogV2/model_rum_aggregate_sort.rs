// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RUMAggregateSort {
    /// An aggregation function.
    #[serde(rename = "aggregation", skip_serializing_if = "Option::is_none")]
    pub aggregation: RUMAggregationFunction,
    /// The metric to sort by (only used for `type=measure`).
    #[serde(rename = "metric", skip_serializing_if = "Option::is_none")]
    pub metric: String,
    /// The order to use, ascending or descending.
    #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
    pub order: RUMSortOrder,
    /// The type of sorting algorithm.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: RUMAggregateSortType,
}


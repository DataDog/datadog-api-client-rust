// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorFormulaAndFunctionEventQueryGroupBySort {
    /// Aggregation methods for event platform queries.
    #[serde(rename = "aggregation", skip_serializing_if = "Option::is_none")]
    pub aggregation: MonitorFormulaAndFunctionEventAggregation,
    /// Metric used for sorting group by results.
    #[serde(rename = "metric", skip_serializing_if = "Option::is_none")]
    pub metric: String,
    /// Direction of sort.
    #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
    pub order: QuerySortOrder,
}


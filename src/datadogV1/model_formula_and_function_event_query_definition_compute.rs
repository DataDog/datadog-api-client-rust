// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FormulaAndFunctionEventQueryDefinitionCompute {
    /// Aggregation methods for event platform queries.
    #[serde(rename = "aggregation", skip_serializing_if = "Option::is_none")]
    pub aggregation: FormulaAndFunctionEventAggregation,
    /// A time interval in milliseconds.
    #[serde(rename = "interval", skip_serializing_if = "Option::is_none")]
    pub interval: i64,
    /// Measurable attribute to compute.
    #[serde(rename = "metric", skip_serializing_if = "Option::is_none")]
    pub metric: String,
}


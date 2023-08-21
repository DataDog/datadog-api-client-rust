// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityMonitoringStandardRuleQuery {
    /// The aggregation type.
    #[serde(rename = "aggregation", skip_serializing_if = "Option::is_none")]
    pub aggregation: SecurityMonitoringRuleQueryAggregation,
    /// Field for which the cardinality is measured. Sent as an array.
    #[serde(rename = "distinctFields", skip_serializing_if = "Option::is_none")]
    pub distinct_fields: Vec<String>,
    /// Fields to group by.
    #[serde(rename = "groupByFields", skip_serializing_if = "Option::is_none")]
    pub group_by_fields: Vec<String>,
    /// (Deprecated) The target field to aggregate over when using the sum or max
aggregations. `metrics` field should be used instead.
    #[serde(rename = "metric", skip_serializing_if = "Option::is_none")]
    pub metric: String,
    /// Group of target fields to aggregate over when using the sum, max, geo data, or new value aggregations. The sum, max, and geo data aggregations only accept one value in this list, whereas the new value aggregation accepts up to five values.
    #[serde(rename = "metrics", skip_serializing_if = "Option::is_none")]
    pub metrics: Vec<String>,
    /// Name of the query.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// Query to run on logs.
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: String,
}


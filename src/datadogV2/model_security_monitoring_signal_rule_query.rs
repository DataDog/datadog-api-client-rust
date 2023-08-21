// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityMonitoringSignalRuleQuery {
    /// The aggregation type.
    #[serde(rename = "aggregation", skip_serializing_if = "Option::is_none")]
    pub aggregation: SecurityMonitoringRuleQueryAggregation,
    /// Fields to group by.
    #[serde(rename = "correlatedByFields", skip_serializing_if = "Option::is_none")]
    pub correlated_by_fields: Vec<String>,
    /// Index of the rule query used to retrieve the correlated field.
    #[serde(rename = "correlatedQueryIndex", skip_serializing_if = "Option::is_none")]
    pub correlated_query_index: i32,
    /// Group of target fields to aggregate over.
    #[serde(rename = "metrics", skip_serializing_if = "Option::is_none")]
    pub metrics: Vec<String>,
    /// Name of the query.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// Rule ID to match on signals.
    #[serde(rename = "ruleId", skip_serializing_if = "Option::is_none")]
    pub rule_id: String,
}


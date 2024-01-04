// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Query for matching rule on signals.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityMonitoringSignalRuleQuery {
    /// The aggregation type.
    #[serde(rename = "aggregation")]
    pub aggregation: Option<crate::datadogV2::model::SecurityMonitoringRuleQueryAggregation>,
    /// Fields to group by.
    #[serde(rename = "correlatedByFields")]
    pub correlated_by_fields: Option<Vec<String>>,
    /// Index of the rule query used to retrieve the correlated field.
    #[serde(rename = "correlatedQueryIndex")]
    pub correlated_query_index: Option<i32>,
    /// Group of target fields to aggregate over.
    #[serde(rename = "metrics")]
    pub metrics: Option<Vec<String>>,
    /// Name of the query.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Rule ID to match on signals.
    #[serde(rename = "ruleId")]
    pub rule_id: String,
}

impl SecurityMonitoringSignalRuleQuery {
    pub fn new(rule_id: String) -> SecurityMonitoringSignalRuleQuery {
        SecurityMonitoringSignalRuleQuery {
            aggregation: None,
            correlated_by_fields: None,
            correlated_query_index: None,
            metrics: None,
            name: None,
            rule_id,
        }
    }
}
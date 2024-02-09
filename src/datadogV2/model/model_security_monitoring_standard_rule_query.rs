// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Query for matching rule.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityMonitoringStandardRuleQuery {
    /// The aggregation type.
    #[serde(rename = "aggregation")]
    pub aggregation: Option<crate::datadogV2::model::SecurityMonitoringRuleQueryAggregation>,
    /// Field for which the cardinality is measured. Sent as an array.
    #[serde(rename = "distinctFields")]
    pub distinct_fields: Option<Vec<String>>,
    /// Fields to group by.
    #[serde(rename = "groupByFields")]
    pub group_by_fields: Option<Vec<String>>,
    /// When false, events without a group-by value are ignored by the rule. When true, events with missing group-by fields are processed with `N/A`, replacing the missing values.
    #[serde(rename = "hasOptionalGroupByFields")]
    pub has_optional_group_by_fields: Option<bool>,
    /// (Deprecated) The target field to aggregate over when using the sum or max
    /// aggregations. `metrics` field should be used instead.
    #[deprecated]
    #[serde(rename = "metric")]
    pub metric: Option<String>,
    /// Group of target fields to aggregate over when using the sum, max, geo data, or new value aggregations. The sum, max, and geo data aggregations only accept one value in this list, whereas the new value aggregation accepts up to five values.
    #[serde(rename = "metrics")]
    pub metrics: Option<Vec<String>>,
    /// Name of the query.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Query to run on logs.
    #[serde(rename = "query")]
    pub query: Option<String>,
}

impl SecurityMonitoringStandardRuleQuery {
    pub fn new() -> SecurityMonitoringStandardRuleQuery {
        #[allow(deprecated)]
        SecurityMonitoringStandardRuleQuery {
            aggregation: None,
            distinct_fields: None,
            group_by_fields: None,
            has_optional_group_by_fields: None,
            metric: None,
            metrics: None,
            name: None,
            query: None,
        }
    }

    #[allow(deprecated)]
    pub fn aggregation(
        &mut self,
        value: crate::datadogV2::model::SecurityMonitoringRuleQueryAggregation,
    ) -> &mut Self {
        self.aggregation = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn distinct_fields(&mut self, value: Vec<String>) -> &mut Self {
        self.distinct_fields = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn group_by_fields(&mut self, value: Vec<String>) -> &mut Self {
        self.group_by_fields = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn has_optional_group_by_fields(&mut self, value: bool) -> &mut Self {
        self.has_optional_group_by_fields = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn metric(&mut self, value: String) -> &mut Self {
        self.metric = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn metrics(&mut self, value: Vec<String>) -> &mut Self {
        self.metrics = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn name(&mut self, value: String) -> &mut Self {
        self.name = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn query(&mut self, value: String) -> &mut Self {
        self.query = Some(value);
        self
    }
}

impl Default for SecurityMonitoringStandardRuleQuery {
    fn default() -> Self {
        Self::new()
    }
}

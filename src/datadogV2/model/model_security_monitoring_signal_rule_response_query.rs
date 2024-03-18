// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Query for matching rule on signals.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringSignalRuleResponseQuery {
    /// The aggregation type.
    #[serde(rename = "aggregation")]
    pub aggregation: Option<crate::datadogV2::model::SecurityMonitoringRuleQueryAggregation>,
    /// Fields to correlate by.
    #[serde(rename = "correlatedByFields")]
    pub correlated_by_fields: Option<Vec<String>>,
    /// Index of the rule query used to retrieve the correlated field.
    #[serde(rename = "correlatedQueryIndex")]
    pub correlated_query_index: Option<i32>,
    /// Default Rule ID to match on signals.
    #[serde(rename = "defaultRuleId")]
    pub default_rule_id: Option<String>,
    /// Field for which the cardinality is measured. Sent as an array.
    #[serde(rename = "distinctFields")]
    pub distinct_fields: Option<Vec<String>>,
    /// Fields to group by.
    #[serde(rename = "groupByFields")]
    pub group_by_fields: Option<Vec<String>>,
    /// Group of target fields to aggregate over.
    #[serde(rename = "metrics")]
    pub metrics: Option<Vec<String>>,
    /// Name of the query.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Rule ID to match on signals.
    #[serde(rename = "ruleId")]
    pub rule_id: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringSignalRuleResponseQuery {
    pub fn new() -> SecurityMonitoringSignalRuleResponseQuery {
        SecurityMonitoringSignalRuleResponseQuery {
            aggregation: None,
            correlated_by_fields: None,
            correlated_query_index: None,
            default_rule_id: None,
            distinct_fields: None,
            group_by_fields: None,
            metrics: None,
            name: None,
            rule_id: None,
            _unparsed: false,
        }
    }

    pub fn aggregation(
        mut self,
        value: crate::datadogV2::model::SecurityMonitoringRuleQueryAggregation,
    ) -> Self {
        self.aggregation = Some(value);
        self
    }

    pub fn correlated_by_fields(mut self, value: Vec<String>) -> Self {
        self.correlated_by_fields = Some(value);
        self
    }

    pub fn correlated_query_index(mut self, value: i32) -> Self {
        self.correlated_query_index = Some(value);
        self
    }

    pub fn default_rule_id(mut self, value: String) -> Self {
        self.default_rule_id = Some(value);
        self
    }

    pub fn distinct_fields(mut self, value: Vec<String>) -> Self {
        self.distinct_fields = Some(value);
        self
    }

    pub fn group_by_fields(mut self, value: Vec<String>) -> Self {
        self.group_by_fields = Some(value);
        self
    }

    pub fn metrics(mut self, value: Vec<String>) -> Self {
        self.metrics = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn rule_id(mut self, value: String) -> Self {
        self.rule_id = Some(value);
        self
    }
}

impl Default for SecurityMonitoringSignalRuleResponseQuery {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SecurityMonitoringSignalRuleResponseQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringSignalRuleResponseQueryVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringSignalRuleResponseQueryVisitor {
            type Value = SecurityMonitoringSignalRuleResponseQuery;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut aggregation: Option<
                    crate::datadogV2::model::SecurityMonitoringRuleQueryAggregation,
                > = None;
                let mut correlated_by_fields: Option<Vec<String>> = None;
                let mut correlated_query_index: Option<i32> = None;
                let mut default_rule_id: Option<String> = None;
                let mut distinct_fields: Option<Vec<String>> = None;
                let mut group_by_fields: Option<Vec<String>> = None;
                let mut metrics: Option<Vec<String>> = None;
                let mut name: Option<String> = None;
                let mut rule_id: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "aggregation" => {
                            if v.is_null() {
                                continue;
                            }
                            aggregation =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _aggregation) = aggregation {
                                match _aggregation {
                                    crate::datadogV2::model::SecurityMonitoringRuleQueryAggregation::UnparsedObject(_aggregation) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "correlatedByFields" => {
                            if v.is_null() {
                                continue;
                            }
                            correlated_by_fields =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "correlatedQueryIndex" => {
                            if v.is_null() {
                                continue;
                            }
                            correlated_query_index =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "defaultRuleId" => {
                            if v.is_null() {
                                continue;
                            }
                            default_rule_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "distinctFields" => {
                            if v.is_null() {
                                continue;
                            }
                            distinct_fields =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "groupByFields" => {
                            if v.is_null() {
                                continue;
                            }
                            group_by_fields =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metrics" => {
                            if v.is_null() {
                                continue;
                            }
                            metrics = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ruleId" => {
                            if v.is_null() {
                                continue;
                            }
                            rule_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SecurityMonitoringSignalRuleResponseQuery {
                    aggregation,
                    correlated_by_fields,
                    correlated_query_index,
                    default_rule_id,
                    distinct_fields,
                    group_by_fields,
                    metrics,
                    name,
                    rule_id,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringSignalRuleResponseQueryVisitor)
    }
}

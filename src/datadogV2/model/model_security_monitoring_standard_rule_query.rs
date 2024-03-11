// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Query for matching rule.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
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
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
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
            _unparsed: false,
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

impl<'de> Deserialize<'de> for SecurityMonitoringStandardRuleQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringStandardRuleQueryVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringStandardRuleQueryVisitor {
            type Value = SecurityMonitoringStandardRuleQuery;

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
                let mut distinct_fields: Option<Vec<String>> = None;
                let mut group_by_fields: Option<Vec<String>> = None;
                let mut has_optional_group_by_fields: Option<bool> = None;
                let mut metric: Option<String> = None;
                let mut metrics: Option<Vec<String>> = None;
                let mut name: Option<String> = None;
                let mut query: Option<String> = None;
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
                        "hasOptionalGroupByFields" => {
                            if v.is_null() {
                                continue;
                            }
                            has_optional_group_by_fields =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metric" => {
                            if v.is_null() {
                                continue;
                            }
                            metric = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "query" => {
                            if v.is_null() {
                                continue;
                            }
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SecurityMonitoringStandardRuleQuery {
                    aggregation,
                    distinct_fields,
                    group_by_fields,
                    has_optional_group_by_fields,
                    metric,
                    metrics,
                    name,
                    query,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringStandardRuleQueryVisitor)
    }
}

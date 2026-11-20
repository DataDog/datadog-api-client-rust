// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// Health across all counters in the active accounting window. Includes hidden counters; contains no scope values or identities.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiGatewayBudgetReportingSummary {
    /// Number of counters at or above capacity.
    #[serde(rename = "exceeded_capacity_count")]
    pub exceeded_capacity_count: i64,
    /// Number of counters in the active window. Counters are created on first use.
    #[serde(rename = "instance_count")]
    pub instance_count: i64,
    /// How the budget limit applies. `shared` is one limit for all matching traffic. `per_instance` is one
/// limit for each counter of a template budget that splits by value.
    #[serde(rename = "limit_mode")]
    pub limit_mode: crate::datadogV2::model::AiGatewayBudgetLimitMode,
    /// Number of counters at or above the warning threshold.
    #[serde(rename = "over_warning_threshold_count")]
    pub over_warning_threshold_count: i64,
    /// Combined capacity of the counters in the active window, in microcents. A shared budget reports its configured limit before its first counter exists.
    #[serde(rename = "total_capacity_microcents")]
    pub total_capacity_microcents: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl AiGatewayBudgetReportingSummary {
    pub fn new(exceeded_capacity_count: i64, instance_count: i64, limit_mode: crate::datadogV2::model::AiGatewayBudgetLimitMode, over_warning_threshold_count: i64, total_capacity_microcents: i64) -> AiGatewayBudgetReportingSummary {
        AiGatewayBudgetReportingSummary {
            exceeded_capacity_count,
            instance_count,
            limit_mode,
            over_warning_threshold_count,
            total_capacity_microcents,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }
    
    pub fn additional_properties(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.additional_properties = value;
        self
    }
}



impl<'de> Deserialize<'de> for AiGatewayBudgetReportingSummary {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AiGatewayBudgetReportingSummaryVisitor;
        impl<'a> Visitor<'a> for AiGatewayBudgetReportingSummaryVisitor {
            type Value = AiGatewayBudgetReportingSummary;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut exceeded_capacity_count: Option<i64> = None;
                let mut instance_count: Option<i64> = None;
                let mut limit_mode: Option<crate::datadogV2::model::AiGatewayBudgetLimitMode> = None;
                let mut over_warning_threshold_count: Option<i64> = None;
                let mut total_capacity_microcents: Option<i64> = None;
                    let mut additional_properties: std::collections::BTreeMap<String, serde_json::Value> = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "exceeded_capacity_count" => {
                            exceeded_capacity_count = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "instance_count" => {
                            instance_count = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "limit_mode" => {
                            limit_mode = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _limit_mode) = limit_mode {
                                match _limit_mode {
                                    crate::datadogV2::model::AiGatewayBudgetLimitMode::UnparsedObject(_limit_mode) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "over_warning_threshold_count" => {
                            over_warning_threshold_count = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "total_capacity_microcents" => {
                            total_capacity_microcents = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        },
                    }
                }
                let exceeded_capacity_count = exceeded_capacity_count.ok_or_else(|| M::Error::missing_field("exceeded_capacity_count"))?;
                let instance_count = instance_count.ok_or_else(|| M::Error::missing_field("instance_count"))?;
                let limit_mode = limit_mode.ok_or_else(|| M::Error::missing_field("limit_mode"))?;
                let over_warning_threshold_count = over_warning_threshold_count.ok_or_else(|| M::Error::missing_field("over_warning_threshold_count"))?;
                let total_capacity_microcents = total_capacity_microcents.ok_or_else(|| M::Error::missing_field("total_capacity_microcents"))?;

                let content = AiGatewayBudgetReportingSummary {
                    exceeded_capacity_count,
                    instance_count,
                    limit_mode,
                    over_warning_threshold_count,
                    total_capacity_microcents,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AiGatewayBudgetReportingSummaryVisitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Aggregate state across budget instance counters.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiGatewayBudgetInstancesSummaryAttributes {
    /// Budget identifier.
    #[serde(rename = "budget_id")]
    pub budget_id: uuid::Uuid,
    /// Number of instances at or above capacity.
    #[serde(rename = "exceeded_capacity_count")]
    pub exceeded_capacity_count: i64,
    /// Number of instance counters.
    #[serde(rename = "instance_count")]
    pub instance_count: i64,
    /// Number of instances at or above the warning threshold.
    #[serde(rename = "over_warning_threshold_count")]
    pub over_warning_threshold_count: i64,
    /// Whether instances were withheld because their provider or model is hidden. Summary counts and totals exclude those instances.
    #[serde(rename = "restricted_details")]
    pub restricted_details: bool,
    /// Spend summed across all instances, in microcents.
    #[serde(rename = "total_spend_microcents")]
    pub total_spend_microcents: i64,
    /// Start of the accounting window.
    #[serde(rename = "window_start")]
    pub window_start: chrono::DateTime<chrono::Utc>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AiGatewayBudgetInstancesSummaryAttributes {
    pub fn new(
        budget_id: uuid::Uuid,
        exceeded_capacity_count: i64,
        instance_count: i64,
        over_warning_threshold_count: i64,
        restricted_details: bool,
        total_spend_microcents: i64,
        window_start: chrono::DateTime<chrono::Utc>,
    ) -> AiGatewayBudgetInstancesSummaryAttributes {
        AiGatewayBudgetInstancesSummaryAttributes {
            budget_id,
            exceeded_capacity_count,
            instance_count,
            over_warning_threshold_count,
            restricted_details,
            total_spend_microcents,
            window_start,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for AiGatewayBudgetInstancesSummaryAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AiGatewayBudgetInstancesSummaryAttributesVisitor;
        impl<'a> Visitor<'a> for AiGatewayBudgetInstancesSummaryAttributesVisitor {
            type Value = AiGatewayBudgetInstancesSummaryAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut budget_id: Option<uuid::Uuid> = None;
                let mut exceeded_capacity_count: Option<i64> = None;
                let mut instance_count: Option<i64> = None;
                let mut over_warning_threshold_count: Option<i64> = None;
                let mut restricted_details: Option<bool> = None;
                let mut total_spend_microcents: Option<i64> = None;
                let mut window_start: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "budget_id" => {
                            budget_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "exceeded_capacity_count" => {
                            exceeded_capacity_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "instance_count" => {
                            instance_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "over_warning_threshold_count" => {
                            over_warning_threshold_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "restricted_details" => {
                            restricted_details =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_spend_microcents" => {
                            total_spend_microcents =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "window_start" => {
                            window_start =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let budget_id = budget_id.ok_or_else(|| M::Error::missing_field("budget_id"))?;
                let exceeded_capacity_count = exceeded_capacity_count
                    .ok_or_else(|| M::Error::missing_field("exceeded_capacity_count"))?;
                let instance_count =
                    instance_count.ok_or_else(|| M::Error::missing_field("instance_count"))?;
                let over_warning_threshold_count = over_warning_threshold_count
                    .ok_or_else(|| M::Error::missing_field("over_warning_threshold_count"))?;
                let restricted_details = restricted_details
                    .ok_or_else(|| M::Error::missing_field("restricted_details"))?;
                let total_spend_microcents = total_spend_microcents
                    .ok_or_else(|| M::Error::missing_field("total_spend_microcents"))?;
                let window_start =
                    window_start.ok_or_else(|| M::Error::missing_field("window_start"))?;

                let content = AiGatewayBudgetInstancesSummaryAttributes {
                    budget_id,
                    exceeded_capacity_count,
                    instance_count,
                    over_warning_threshold_count,
                    restricted_details,
                    total_spend_microcents,
                    window_start,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AiGatewayBudgetInstancesSummaryAttributesVisitor)
    }
}

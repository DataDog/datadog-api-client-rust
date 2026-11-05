// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Accounting state enforced for an AI Gateway budget's active period.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiGatewayBudgetPeriodAttributes {
    /// Budget resource identifier.
    #[serde(rename = "budget_id")]
    pub budget_id: uuid::Uuid,
    /// Spend currently counted against the active accounting period.
    #[serde(rename = "current_spend_microcents")]
    pub current_spend_microcents: i64,
    /// Capacity that new accounting periods inherit from the budget definition.
    #[serde(rename = "default_capacity_microcents")]
    pub default_capacity_microcents: i64,
    /// Capacity enforced for the active accounting period.
    #[serde(rename = "effective_capacity_microcents")]
    pub effective_capacity_microcents: i64,
    /// Datadog monitor ID provisioned for this budget. Null when monitor provisioning was skipped or failed.
    #[serialize_always]
    #[serde(rename = "monitor_id")]
    pub monitor_id: Option<i64>,
    /// End of the active accounting window.
    #[serde(rename = "window_end")]
    pub window_end: chrono::DateTime<chrono::Utc>,
    /// Start of the active accounting window.
    #[serde(rename = "window_start")]
    pub window_start: chrono::DateTime<chrono::Utc>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AiGatewayBudgetPeriodAttributes {
    pub fn new(
        budget_id: uuid::Uuid,
        current_spend_microcents: i64,
        default_capacity_microcents: i64,
        effective_capacity_microcents: i64,
        monitor_id: Option<i64>,
        window_end: chrono::DateTime<chrono::Utc>,
        window_start: chrono::DateTime<chrono::Utc>,
    ) -> AiGatewayBudgetPeriodAttributes {
        AiGatewayBudgetPeriodAttributes {
            budget_id,
            current_spend_microcents,
            default_capacity_microcents,
            effective_capacity_microcents,
            monitor_id,
            window_end,
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

impl<'de> Deserialize<'de> for AiGatewayBudgetPeriodAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AiGatewayBudgetPeriodAttributesVisitor;
        impl<'a> Visitor<'a> for AiGatewayBudgetPeriodAttributesVisitor {
            type Value = AiGatewayBudgetPeriodAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut budget_id: Option<uuid::Uuid> = None;
                let mut current_spend_microcents: Option<i64> = None;
                let mut default_capacity_microcents: Option<i64> = None;
                let mut effective_capacity_microcents: Option<i64> = None;
                let mut monitor_id: Option<Option<i64>> = None;
                let mut window_end: Option<chrono::DateTime<chrono::Utc>> = None;
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
                        "current_spend_microcents" => {
                            current_spend_microcents =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "default_capacity_microcents" => {
                            default_capacity_microcents =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "effective_capacity_microcents" => {
                            effective_capacity_microcents =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "monitor_id" => {
                            monitor_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "window_end" => {
                            window_end = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let current_spend_microcents = current_spend_microcents
                    .ok_or_else(|| M::Error::missing_field("current_spend_microcents"))?;
                let default_capacity_microcents = default_capacity_microcents
                    .ok_or_else(|| M::Error::missing_field("default_capacity_microcents"))?;
                let effective_capacity_microcents = effective_capacity_microcents
                    .ok_or_else(|| M::Error::missing_field("effective_capacity_microcents"))?;
                let monitor_id = monitor_id.ok_or_else(|| M::Error::missing_field("monitor_id"))?;
                let window_end = window_end.ok_or_else(|| M::Error::missing_field("window_end"))?;
                let window_start =
                    window_start.ok_or_else(|| M::Error::missing_field("window_start"))?;

                let content = AiGatewayBudgetPeriodAttributes {
                    budget_id,
                    current_spend_microcents,
                    default_capacity_microcents,
                    effective_capacity_microcents,
                    monitor_id,
                    window_end,
                    window_start,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AiGatewayBudgetPeriodAttributesVisitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// One budget instance counter.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiGatewayBudgetInstanceAttributes {
    /// Budget identifier.
    #[serde(rename = "budget_id")]
    pub budget_id: uuid::Uuid,
    /// Capacity enforced for this instance, in microcents.
    #[serde(rename = "effective_capacity_microcents")]
    pub effective_capacity_microcents: i64,
    /// Opaque canonical counter key. Empty for legacy pooled counters.
    #[serde(rename = "instance_key")]
    pub instance_key: String,
    /// Time the instance counter was last modified.
    #[serde(rename = "modified_at")]
    pub modified_at: chrono::DateTime<chrono::Utc>,
    /// Spend counted against this instance, in microcents.
    #[serde(rename = "spend_microcents")]
    pub spend_microcents: i64,
    /// Dimension values decoded from instance_key. User and Team values are raw UUIDs.
    #[serde(rename = "split_dims")]
    pub split_dims: std::collections::BTreeMap<String, String>,
    /// Start of the accounting window.
    #[serde(rename = "window_start")]
    pub window_start: chrono::DateTime<chrono::Utc>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AiGatewayBudgetInstanceAttributes {
    pub fn new(
        budget_id: uuid::Uuid,
        effective_capacity_microcents: i64,
        instance_key: String,
        modified_at: chrono::DateTime<chrono::Utc>,
        spend_microcents: i64,
        split_dims: std::collections::BTreeMap<String, String>,
        window_start: chrono::DateTime<chrono::Utc>,
    ) -> AiGatewayBudgetInstanceAttributes {
        AiGatewayBudgetInstanceAttributes {
            budget_id,
            effective_capacity_microcents,
            instance_key,
            modified_at,
            spend_microcents,
            split_dims,
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

impl<'de> Deserialize<'de> for AiGatewayBudgetInstanceAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AiGatewayBudgetInstanceAttributesVisitor;
        impl<'a> Visitor<'a> for AiGatewayBudgetInstanceAttributesVisitor {
            type Value = AiGatewayBudgetInstanceAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut budget_id: Option<uuid::Uuid> = None;
                let mut effective_capacity_microcents: Option<i64> = None;
                let mut instance_key: Option<String> = None;
                let mut modified_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut spend_microcents: Option<i64> = None;
                let mut split_dims: Option<std::collections::BTreeMap<String, String>> = None;
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
                        "effective_capacity_microcents" => {
                            effective_capacity_microcents =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "instance_key" => {
                            instance_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_at" => {
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "spend_microcents" => {
                            spend_microcents =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "split_dims" => {
                            split_dims = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let effective_capacity_microcents = effective_capacity_microcents
                    .ok_or_else(|| M::Error::missing_field("effective_capacity_microcents"))?;
                let instance_key =
                    instance_key.ok_or_else(|| M::Error::missing_field("instance_key"))?;
                let modified_at =
                    modified_at.ok_or_else(|| M::Error::missing_field("modified_at"))?;
                let spend_microcents =
                    spend_microcents.ok_or_else(|| M::Error::missing_field("spend_microcents"))?;
                let split_dims = split_dims.ok_or_else(|| M::Error::missing_field("split_dims"))?;
                let window_start =
                    window_start.ok_or_else(|| M::Error::missing_field("window_start"))?;

                let content = AiGatewayBudgetInstanceAttributes {
                    budget_id,
                    effective_capacity_microcents,
                    instance_key,
                    modified_at,
                    spend_microcents,
                    split_dims,
                    window_start,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AiGatewayBudgetInstanceAttributesVisitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Values of one budget split dimension.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiGatewayBudgetInstanceDimensionAttributes {
    /// Budget identifier.
    #[serde(rename = "budget_id")]
    pub budget_id: uuid::Uuid,
    /// Split dimension name, as used in `split_dims` and the `split_dim` filter.
    #[serde(rename = "dim")]
    pub dim: String,
    /// Whether identifying instance details were withheld. Aggregate totals still include restricted instances.
    #[serde(rename = "restricted_details")]
    pub restricted_details: bool,
    /// Whether more values matched than were returned. Narrow the results with `query` or `value` to see them.
    #[serde(rename = "truncated")]
    pub truncated: bool,
    /// Values of this dimension across matching instance counters, highest spend first and at most `limit` of them. Values are counted under every `split_dim` filter except the one for this dimension.
    #[serde(rename = "values")]
    pub values: Vec<crate::datadogV2::model::AiGatewayBudgetInstanceDimensionValue>,
    /// Start of the accounting window.
    #[serde(rename = "window_start")]
    pub window_start: chrono::DateTime<chrono::Utc>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AiGatewayBudgetInstanceDimensionAttributes {
    pub fn new(
        budget_id: uuid::Uuid,
        dim: String,
        restricted_details: bool,
        truncated: bool,
        values: Vec<crate::datadogV2::model::AiGatewayBudgetInstanceDimensionValue>,
        window_start: chrono::DateTime<chrono::Utc>,
    ) -> AiGatewayBudgetInstanceDimensionAttributes {
        AiGatewayBudgetInstanceDimensionAttributes {
            budget_id,
            dim,
            restricted_details,
            truncated,
            values,
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

impl<'de> Deserialize<'de> for AiGatewayBudgetInstanceDimensionAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AiGatewayBudgetInstanceDimensionAttributesVisitor;
        impl<'a> Visitor<'a> for AiGatewayBudgetInstanceDimensionAttributesVisitor {
            type Value = AiGatewayBudgetInstanceDimensionAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut budget_id: Option<uuid::Uuid> = None;
                let mut dim: Option<String> = None;
                let mut restricted_details: Option<bool> = None;
                let mut truncated: Option<bool> = None;
                let mut values: Option<
                    Vec<crate::datadogV2::model::AiGatewayBudgetInstanceDimensionValue>,
                > = None;
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
                        "dim" => {
                            dim = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "restricted_details" => {
                            restricted_details =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "truncated" => {
                            truncated = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "values" => {
                            values = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let dim = dim.ok_or_else(|| M::Error::missing_field("dim"))?;
                let restricted_details = restricted_details
                    .ok_or_else(|| M::Error::missing_field("restricted_details"))?;
                let truncated = truncated.ok_or_else(|| M::Error::missing_field("truncated"))?;
                let values = values.ok_or_else(|| M::Error::missing_field("values"))?;
                let window_start =
                    window_start.ok_or_else(|| M::Error::missing_field("window_start"))?;

                let content = AiGatewayBudgetInstanceDimensionAttributes {
                    budget_id,
                    dim,
                    restricted_details,
                    truncated,
                    values,
                    window_start,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AiGatewayBudgetInstanceDimensionAttributesVisitor)
    }
}

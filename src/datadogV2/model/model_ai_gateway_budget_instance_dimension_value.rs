// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// One value of a budget split dimension.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiGatewayBudgetInstanceDimensionValue {
    /// Number of instance counters carrying this value.
    #[serde(rename = "instance_count")]
    pub instance_count: i64,
    /// Spend summed across those instance counters, in microcents.
    #[serde(rename = "spend_microcents")]
    pub spend_microcents: i64,
    /// Raw dimension value. User and Team values are UUIDs. A counter with no value for the dimension carries the null character (U+0000).
    #[serde(rename = "value")]
    pub value: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AiGatewayBudgetInstanceDimensionValue {
    pub fn new(
        instance_count: i64,
        spend_microcents: i64,
        value: String,
    ) -> AiGatewayBudgetInstanceDimensionValue {
        AiGatewayBudgetInstanceDimensionValue {
            instance_count,
            spend_microcents,
            value,
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

impl<'de> Deserialize<'de> for AiGatewayBudgetInstanceDimensionValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AiGatewayBudgetInstanceDimensionValueVisitor;
        impl<'a> Visitor<'a> for AiGatewayBudgetInstanceDimensionValueVisitor {
            type Value = AiGatewayBudgetInstanceDimensionValue;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut instance_count: Option<i64> = None;
                let mut spend_microcents: Option<i64> = None;
                let mut value: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "instance_count" => {
                            instance_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "spend_microcents" => {
                            spend_microcents =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "value" => {
                            value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let instance_count =
                    instance_count.ok_or_else(|| M::Error::missing_field("instance_count"))?;
                let spend_microcents =
                    spend_microcents.ok_or_else(|| M::Error::missing_field("spend_microcents"))?;
                let value = value.ok_or_else(|| M::Error::missing_field("value"))?;

                let content = AiGatewayBudgetInstanceDimensionValue {
                    instance_count,
                    spend_microcents,
                    value,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AiGatewayBudgetInstanceDimensionValueVisitor)
    }
}

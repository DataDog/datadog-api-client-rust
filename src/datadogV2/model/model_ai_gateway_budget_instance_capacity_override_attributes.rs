// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes used to override one budget instance's active-period capacity.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiGatewayBudgetInstanceCapacityOverrideAttributes {
    /// New capacity for this instance in the active accounting period, in microcents.
    #[serde(rename = "capacity_microcents")]
    pub capacity_microcents: i64,
    /// Counter key of the instance to override, as returned by the list budget instances endpoint.
    #[serde(rename = "instance_key")]
    pub instance_key: String,
    /// Optional reason recorded with the capacity override.
    #[serde(rename = "reason")]
    pub reason: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AiGatewayBudgetInstanceCapacityOverrideAttributes {
    pub fn new(
        capacity_microcents: i64,
        instance_key: String,
    ) -> AiGatewayBudgetInstanceCapacityOverrideAttributes {
        AiGatewayBudgetInstanceCapacityOverrideAttributes {
            capacity_microcents,
            instance_key,
            reason: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn reason(mut self, value: String) -> Self {
        self.reason = Some(value);
        self
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for AiGatewayBudgetInstanceCapacityOverrideAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AiGatewayBudgetInstanceCapacityOverrideAttributesVisitor;
        impl<'a> Visitor<'a> for AiGatewayBudgetInstanceCapacityOverrideAttributesVisitor {
            type Value = AiGatewayBudgetInstanceCapacityOverrideAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut capacity_microcents: Option<i64> = None;
                let mut instance_key: Option<String> = None;
                let mut reason: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "capacity_microcents" => {
                            capacity_microcents =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "instance_key" => {
                            instance_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "reason" => {
                            if v.is_null() {
                                continue;
                            }
                            reason = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let capacity_microcents = capacity_microcents
                    .ok_or_else(|| M::Error::missing_field("capacity_microcents"))?;
                let instance_key =
                    instance_key.ok_or_else(|| M::Error::missing_field("instance_key"))?;

                let content = AiGatewayBudgetInstanceCapacityOverrideAttributes {
                    capacity_microcents,
                    instance_key,
                    reason,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AiGatewayBudgetInstanceCapacityOverrideAttributesVisitor)
    }
}

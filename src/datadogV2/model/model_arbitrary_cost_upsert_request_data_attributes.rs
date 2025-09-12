// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `ArbitraryCostUpsertRequestDataAttributes` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ArbitraryCostUpsertRequestDataAttributes {
    /// The `attributes` `costs_to_allocate`.
    #[serde(rename = "costs_to_allocate")]
    pub costs_to_allocate:
        Vec<crate::datadogV2::model::ArbitraryCostUpsertRequestDataAttributesCostsToAllocateItems>,
    /// The `attributes` `enabled`.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// The `attributes` `order_id`.
    #[serde(rename = "order_id")]
    pub order_id: Option<i64>,
    /// The `attributes` `provider`.
    #[serde(rename = "provider")]
    pub provider: Vec<String>,
    /// The `attributes` `rejected`.
    #[serde(rename = "rejected")]
    pub rejected: Option<bool>,
    /// The `attributes` `rule_name`.
    #[serde(rename = "rule_name")]
    pub rule_name: String,
    /// The definition of `ArbitraryCostUpsertRequestDataAttributesStrategy` object.
    #[serde(rename = "strategy")]
    pub strategy: crate::datadogV2::model::ArbitraryCostUpsertRequestDataAttributesStrategy,
    /// The `attributes` `type`.
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ArbitraryCostUpsertRequestDataAttributes {
    pub fn new(
        costs_to_allocate: Vec<
            crate::datadogV2::model::ArbitraryCostUpsertRequestDataAttributesCostsToAllocateItems,
        >,
        provider: Vec<String>,
        rule_name: String,
        strategy: crate::datadogV2::model::ArbitraryCostUpsertRequestDataAttributesStrategy,
        type_: String,
    ) -> ArbitraryCostUpsertRequestDataAttributes {
        ArbitraryCostUpsertRequestDataAttributes {
            costs_to_allocate,
            enabled: None,
            order_id: None,
            provider,
            rejected: None,
            rule_name,
            strategy,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn order_id(mut self, value: i64) -> Self {
        self.order_id = Some(value);
        self
    }

    pub fn rejected(mut self, value: bool) -> Self {
        self.rejected = Some(value);
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

impl<'de> Deserialize<'de> for ArbitraryCostUpsertRequestDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ArbitraryCostUpsertRequestDataAttributesVisitor;
        impl<'a> Visitor<'a> for ArbitraryCostUpsertRequestDataAttributesVisitor {
            type Value = ArbitraryCostUpsertRequestDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut costs_to_allocate: Option<Vec<crate::datadogV2::model::ArbitraryCostUpsertRequestDataAttributesCostsToAllocateItems>> = None;
                let mut enabled: Option<bool> = None;
                let mut order_id: Option<i64> = None;
                let mut provider: Option<Vec<String>> = None;
                let mut rejected: Option<bool> = None;
                let mut rule_name: Option<String> = None;
                let mut strategy: Option<
                    crate::datadogV2::model::ArbitraryCostUpsertRequestDataAttributesStrategy,
                > = None;
                let mut type_: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "costs_to_allocate" => {
                            costs_to_allocate =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "order_id" => {
                            if v.is_null() {
                                continue;
                            }
                            order_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "provider" => {
                            provider = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rejected" => {
                            if v.is_null() {
                                continue;
                            }
                            rejected = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rule_name" => {
                            rule_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "strategy" => {
                            strategy = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let costs_to_allocate = costs_to_allocate
                    .ok_or_else(|| M::Error::missing_field("costs_to_allocate"))?;
                let provider = provider.ok_or_else(|| M::Error::missing_field("provider"))?;
                let rule_name = rule_name.ok_or_else(|| M::Error::missing_field("rule_name"))?;
                let strategy = strategy.ok_or_else(|| M::Error::missing_field("strategy"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = ArbitraryCostUpsertRequestDataAttributes {
                    costs_to_allocate,
                    enabled,
                    order_id,
                    provider,
                    rejected,
                    rule_name,
                    strategy,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ArbitraryCostUpsertRequestDataAttributesVisitor)
    }
}

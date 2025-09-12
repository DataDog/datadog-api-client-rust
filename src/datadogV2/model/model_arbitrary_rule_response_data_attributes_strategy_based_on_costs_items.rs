// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `ArbitraryRuleResponseDataAttributesStrategyBasedOnCostsItems` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ArbitraryRuleResponseDataAttributesStrategyBasedOnCostsItems {
    /// The `items` `condition`.
    #[serde(rename = "condition")]
    pub condition: String,
    /// The `items` `tag`.
    #[serde(rename = "tag")]
    pub tag: String,
    /// The `items` `value`.
    #[serde(rename = "value")]
    pub value: Option<String>,
    /// The `items` `values`.
    #[serde(rename = "values", default, with = "::serde_with::rust::double_option")]
    pub values: Option<Option<Vec<String>>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ArbitraryRuleResponseDataAttributesStrategyBasedOnCostsItems {
    pub fn new(
        condition: String,
        tag: String,
    ) -> ArbitraryRuleResponseDataAttributesStrategyBasedOnCostsItems {
        ArbitraryRuleResponseDataAttributesStrategyBasedOnCostsItems {
            condition,
            tag,
            value: None,
            values: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn value(mut self, value: String) -> Self {
        self.value = Some(value);
        self
    }

    pub fn values(mut self, value: Option<Vec<String>>) -> Self {
        self.values = Some(value);
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

impl<'de> Deserialize<'de> for ArbitraryRuleResponseDataAttributesStrategyBasedOnCostsItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ArbitraryRuleResponseDataAttributesStrategyBasedOnCostsItemsVisitor;
        impl<'a> Visitor<'a> for ArbitraryRuleResponseDataAttributesStrategyBasedOnCostsItemsVisitor {
            type Value = ArbitraryRuleResponseDataAttributesStrategyBasedOnCostsItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut condition: Option<String> = None;
                let mut tag: Option<String> = None;
                let mut value: Option<String> = None;
                let mut values: Option<Option<Vec<String>>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "condition" => {
                            condition = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tag" => {
                            tag = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "value" => {
                            if v.is_null() {
                                continue;
                            }
                            value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "values" => {
                            values = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let condition = condition.ok_or_else(|| M::Error::missing_field("condition"))?;
                let tag = tag.ok_or_else(|| M::Error::missing_field("tag"))?;

                let content = ArbitraryRuleResponseDataAttributesStrategyBasedOnCostsItems {
                    condition,
                    tag,
                    value,
                    values,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(ArbitraryRuleResponseDataAttributesStrategyBasedOnCostsItemsVisitor)
    }
}

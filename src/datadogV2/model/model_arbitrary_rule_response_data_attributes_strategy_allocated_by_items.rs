// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `ArbitraryRuleResponseDataAttributesStrategyAllocatedByItems` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ArbitraryRuleResponseDataAttributesStrategyAllocatedByItems {
    /// The `items` `allocated_tags`.
    #[serde(rename = "allocated_tags")]
    pub allocated_tags: Vec<crate::datadogV2::model::ArbitraryRuleResponseDataAttributesStrategyAllocatedByItemsAllocatedTagsItems>,
    /// The `items` `percentage`. The numeric value format should be a 32bit float value.
    #[serde(rename = "percentage")]
    pub percentage: f64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ArbitraryRuleResponseDataAttributesStrategyAllocatedByItems {
    pub fn new(
        allocated_tags: Vec<crate::datadogV2::model::ArbitraryRuleResponseDataAttributesStrategyAllocatedByItemsAllocatedTagsItems>,
        percentage: f64,
    ) -> ArbitraryRuleResponseDataAttributesStrategyAllocatedByItems {
        ArbitraryRuleResponseDataAttributesStrategyAllocatedByItems {
            allocated_tags,
            percentage,
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

impl<'de> Deserialize<'de> for ArbitraryRuleResponseDataAttributesStrategyAllocatedByItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ArbitraryRuleResponseDataAttributesStrategyAllocatedByItemsVisitor;
        impl<'a> Visitor<'a> for ArbitraryRuleResponseDataAttributesStrategyAllocatedByItemsVisitor {
            type Value = ArbitraryRuleResponseDataAttributesStrategyAllocatedByItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut allocated_tags: Option<Vec<crate::datadogV2::model::ArbitraryRuleResponseDataAttributesStrategyAllocatedByItemsAllocatedTagsItems>> = None;
                let mut percentage: Option<f64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "allocated_tags" => {
                            allocated_tags =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "percentage" => {
                            percentage = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let allocated_tags =
                    allocated_tags.ok_or_else(|| M::Error::missing_field("allocated_tags"))?;
                let percentage = percentage.ok_or_else(|| M::Error::missing_field("percentage"))?;

                let content = ArbitraryRuleResponseDataAttributesStrategyAllocatedByItems {
                    allocated_tags,
                    percentage,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(ArbitraryRuleResponseDataAttributesStrategyAllocatedByItemsVisitor)
    }
}

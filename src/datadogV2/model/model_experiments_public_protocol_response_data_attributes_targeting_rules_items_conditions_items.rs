// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// One condition in a protocol targeting rule.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsPublicProtocolResponseDataAttributesTargetingRulesItemsConditionsItems {
    /// Subject attribute evaluated by the targeting condition.
    #[serde(rename = "attribute")]
    pub attribute: Option<String>,
    /// Comparison applied to the subject attribute.
    #[serde(rename = "operator")]
    pub operator: Option<String>,
    /// Position of this entry in the ordered configuration.
    #[serde(rename = "order_position")]
    pub order_position: Option<i64>,
    /// ID of the saved filter used by this targeting condition.
    #[serde(rename = "saved_filter_id")]
    pub saved_filter_id: Option<String>,
    /// Values compared with the subject attribute in this condition.
    #[serde(rename = "value")]
    pub value: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExperimentsPublicProtocolResponseDataAttributesTargetingRulesItemsConditionsItems {
    pub fn new() -> ExperimentsPublicProtocolResponseDataAttributesTargetingRulesItemsConditionsItems
    {
        ExperimentsPublicProtocolResponseDataAttributesTargetingRulesItemsConditionsItems {
            attribute: None,
            operator: None,
            order_position: None,
            saved_filter_id: None,
            value: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn attribute(mut self, value: String) -> Self {
        self.attribute = Some(value);
        self
    }

    pub fn operator(mut self, value: String) -> Self {
        self.operator = Some(value);
        self
    }

    pub fn order_position(mut self, value: i64) -> Self {
        self.order_position = Some(value);
        self
    }

    pub fn saved_filter_id(mut self, value: String) -> Self {
        self.saved_filter_id = Some(value);
        self
    }

    pub fn value(mut self, value: Vec<String>) -> Self {
        self.value = Some(value);
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

impl Default for ExperimentsPublicProtocolResponseDataAttributesTargetingRulesItemsConditionsItems {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de>
    for ExperimentsPublicProtocolResponseDataAttributesTargetingRulesItemsConditionsItems
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsPublicProtocolResponseDataAttributesTargetingRulesItemsConditionsItemsVisitor;
        impl<'a> Visitor<'a> for ExperimentsPublicProtocolResponseDataAttributesTargetingRulesItemsConditionsItemsVisitor {
            type Value = ExperimentsPublicProtocolResponseDataAttributesTargetingRulesItemsConditionsItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attribute: Option<String> = None;
                let mut operator: Option<String> = None;
                let mut order_position: Option<i64> = None;
                let mut saved_filter_id: Option<String> = None;
                let mut value: Option<Vec<String>> = None;
                    let mut additional_properties: std::collections::BTreeMap<String, serde_json::Value> = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "attribute" => {
                            if v.is_null() {
                                continue;
                            }
                            attribute = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "operator" => {
                            if v.is_null() {
                                continue;
                            }
                            operator = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "order_position" => {
                            if v.is_null() {
                                continue;
                            }
                            order_position = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "saved_filter_id" => {
                            if v.is_null() {
                                continue;
                            }
                            saved_filter_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "value" => {
                            if v.is_null() {
                                continue;
                            }
                            value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        },
                    }
                }

                let content = ExperimentsPublicProtocolResponseDataAttributesTargetingRulesItemsConditionsItems {
                    attribute,
                    operator,
                    order_position,
                    saved_filter_id,
                    value,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExperimentsPublicProtocolResponseDataAttributesTargetingRulesItemsConditionsItemsVisitor)
    }
}

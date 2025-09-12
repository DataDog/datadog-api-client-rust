// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `UpdateRulesetRequestDataAttributesRulesItemsReferenceTableFieldPairsItems` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UpdateRulesetRequestDataAttributesRulesItemsReferenceTableFieldPairsItems {
    /// The `items` `input_column`.
    #[serde(rename = "input_column")]
    pub input_column: String,
    /// The `items` `output_key`.
    #[serde(rename = "output_key")]
    pub output_key: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UpdateRulesetRequestDataAttributesRulesItemsReferenceTableFieldPairsItems {
    pub fn new(
        input_column: String,
        output_key: String,
    ) -> UpdateRulesetRequestDataAttributesRulesItemsReferenceTableFieldPairsItems {
        UpdateRulesetRequestDataAttributesRulesItemsReferenceTableFieldPairsItems {
            input_column,
            output_key,
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

impl<'de> Deserialize<'de>
    for UpdateRulesetRequestDataAttributesRulesItemsReferenceTableFieldPairsItems
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UpdateRulesetRequestDataAttributesRulesItemsReferenceTableFieldPairsItemsVisitor;
        impl<'a> Visitor<'a>
            for UpdateRulesetRequestDataAttributesRulesItemsReferenceTableFieldPairsItemsVisitor
        {
            type Value = UpdateRulesetRequestDataAttributesRulesItemsReferenceTableFieldPairsItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut input_column: Option<String> = None;
                let mut output_key: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "input_column" => {
                            input_column =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "output_key" => {
                            output_key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let input_column =
                    input_column.ok_or_else(|| M::Error::missing_field("input_column"))?;
                let output_key = output_key.ok_or_else(|| M::Error::missing_field("output_key"))?;

                let content =
                    UpdateRulesetRequestDataAttributesRulesItemsReferenceTableFieldPairsItems {
                        input_column,
                        output_key,
                        additional_properties,
                        _unparsed,
                    };

                Ok(content)
            }
        }

        deserializer.deserialize_any(
            UpdateRulesetRequestDataAttributesRulesItemsReferenceTableFieldPairsItemsVisitor,
        )
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `CompletionCondition` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CompletionCondition {
    /// The `CompletionCondition` `operand1`.
    #[serde(rename = "operand1")]
    pub operand1: serde_json::Value,
    /// The `CompletionCondition` `operand2`.
    #[serde(rename = "operand2")]
    pub operand2: Option<serde_json::Value>,
    /// The definition of `CompletionConditionOperator` object.
    #[serde(rename = "operator")]
    pub operator: crate::datadogV2::model::CompletionConditionOperator,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CompletionCondition {
    pub fn new(
        operand1: serde_json::Value,
        operator: crate::datadogV2::model::CompletionConditionOperator,
    ) -> CompletionCondition {
        CompletionCondition {
            operand1,
            operand2: None,
            operator,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn operand2(mut self, value: serde_json::Value) -> Self {
        self.operand2 = Some(value);
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

impl<'de> Deserialize<'de> for CompletionCondition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CompletionConditionVisitor;
        impl<'a> Visitor<'a> for CompletionConditionVisitor {
            type Value = CompletionCondition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut operand1: Option<serde_json::Value> = None;
                let mut operand2: Option<serde_json::Value> = None;
                let mut operator: Option<crate::datadogV2::model::CompletionConditionOperator> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "operand1" => {
                            operand1 = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "operand2" => {
                            if v.is_null() {
                                continue;
                            }
                            operand2 = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "operator" => {
                            operator = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _operator) = operator {
                                match _operator {
                                    crate::datadogV2::model::CompletionConditionOperator::UnparsedObject(_operator) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let operand1 = operand1.ok_or_else(|| M::Error::missing_field("operand1"))?;
                let operator = operator.ok_or_else(|| M::Error::missing_field("operator"))?;

                let content = CompletionCondition {
                    operand1,
                    operand2,
                    operator,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CompletionConditionVisitor)
    }
}

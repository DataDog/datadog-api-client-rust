// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Condition request payload for targeting rules. A condition is either an inline
/// predicate with `operator`, `attribute`, and `value`, or a reference to a
/// saved filter with `saved_filter_id`. The two shapes are mutually exclusive.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ConditionRequest {
    /// The user or request attribute to evaluate. Required for inline conditions; omit when `saved_filter_id` is set.
    #[serde(rename = "attribute")]
    pub attribute: Option<String>,
    /// The operator used in a targeting condition.
    #[serde(rename = "operator")]
    pub operator: Option<crate::datadogV2::model::ConditionOperator>,
    /// The ID of a saved filter to reference as this condition. Mutually exclusive
    /// with `operator`, `attribute`, and `value`. When set, the saved filter's
    /// targeting rules are evaluated in place of an inline predicate.
    #[serde(rename = "saved_filter_id")]
    pub saved_filter_id: Option<uuid::Uuid>,
    /// Values used by the selected operator. Required for inline conditions; omit when `saved_filter_id` is set.
    #[serde(rename = "value")]
    pub value: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ConditionRequest {
    pub fn new() -> ConditionRequest {
        ConditionRequest {
            attribute: None,
            operator: None,
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

    pub fn operator(mut self, value: crate::datadogV2::model::ConditionOperator) -> Self {
        self.operator = Some(value);
        self
    }

    pub fn saved_filter_id(mut self, value: uuid::Uuid) -> Self {
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

impl Default for ConditionRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ConditionRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ConditionRequestVisitor;
        impl<'a> Visitor<'a> for ConditionRequestVisitor {
            type Value = ConditionRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attribute: Option<String> = None;
                let mut operator: Option<crate::datadogV2::model::ConditionOperator> = None;
                let mut saved_filter_id: Option<uuid::Uuid> = None;
                let mut value: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "attribute" => {
                            if v.is_null() {
                                continue;
                            }
                            attribute = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "operator" => {
                            if v.is_null() {
                                continue;
                            }
                            operator = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _operator) = operator {
                                match _operator {
                                    crate::datadogV2::model::ConditionOperator::UnparsedObject(
                                        _operator,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "saved_filter_id" => {
                            if v.is_null() {
                                continue;
                            }
                            saved_filter_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "value" => {
                            if v.is_null() {
                                continue;
                            }
                            value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ConditionRequest {
                    attribute,
                    operator,
                    saved_filter_id,
                    value,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ConditionRequestVisitor)
    }
}

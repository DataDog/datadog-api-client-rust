// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Composed target for `validatesJSONPath` operator.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsAssertionJSONPathTargetTarget {
    /// The element from the list of results to assert on.  To choose from the first element in the list `firstElementMatches`, every element in the list `everyElementMatches`, at least one element in the list `atLeastOneElementMatches` or the serialized value of the list `serializationMatches`.
    #[serde(rename = "elementsOperator")]
    pub elements_operator: Option<String>,
    /// The JSON path to assert.
    #[serde(rename = "jsonPath")]
    pub json_path: Option<String>,
    /// The specific operator to use on the path.
    #[serde(rename = "operator")]
    pub operator: Option<String>,
    /// The path target value to compare to.
    #[serde(rename = "targetValue")]
    pub target_value: Option<serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsAssertionJSONPathTargetTarget {
    pub fn new() -> SyntheticsAssertionJSONPathTargetTarget {
        SyntheticsAssertionJSONPathTargetTarget {
            elements_operator: None,
            json_path: None,
            operator: None,
            target_value: None,
            _unparsed: false,
        }
    }

    pub fn elements_operator(mut self, value: String) -> Self {
        self.elements_operator = Some(value);
        self
    }

    pub fn json_path(mut self, value: String) -> Self {
        self.json_path = Some(value);
        self
    }

    pub fn operator(mut self, value: String) -> Self {
        self.operator = Some(value);
        self
    }

    pub fn target_value(mut self, value: serde_json::Value) -> Self {
        self.target_value = Some(value);
        self
    }
}

impl Default for SyntheticsAssertionJSONPathTargetTarget {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsAssertionJSONPathTargetTarget {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsAssertionJSONPathTargetTargetVisitor;
        impl<'a> Visitor<'a> for SyntheticsAssertionJSONPathTargetTargetVisitor {
            type Value = SyntheticsAssertionJSONPathTargetTarget;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut elements_operator: Option<String> = None;
                let mut json_path: Option<String> = None;
                let mut operator: Option<String> = None;
                let mut target_value: Option<serde_json::Value> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "elementsOperator" => {
                            if v.is_null() {
                                continue;
                            }
                            elements_operator =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "jsonPath" => {
                            if v.is_null() {
                                continue;
                            }
                            json_path = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "operator" => {
                            if v.is_null() {
                                continue;
                            }
                            operator = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "targetValue" => {
                            if v.is_null() {
                                continue;
                            }
                            target_value =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SyntheticsAssertionJSONPathTargetTarget {
                    elements_operator,
                    json_path,
                    operator,
                    target_value,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsAssertionJSONPathTargetTargetVisitor)
    }
}

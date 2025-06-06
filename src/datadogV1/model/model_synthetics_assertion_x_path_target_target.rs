// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Composed target for `validatesXPath` operator.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsAssertionXPathTargetTarget {
    /// The specific operator to use on the path.
    #[serde(rename = "operator")]
    pub operator: Option<String>,
    /// Value used by the operator in assertions. Can be either a number or string.
    #[serde(rename = "targetValue")]
    pub target_value: Option<crate::datadogV1::model::SyntheticsAssertionTargetValue>,
    /// The X path to assert.
    #[serde(rename = "xPath")]
    pub x_path: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsAssertionXPathTargetTarget {
    pub fn new() -> SyntheticsAssertionXPathTargetTarget {
        SyntheticsAssertionXPathTargetTarget {
            operator: None,
            target_value: None,
            x_path: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn operator(mut self, value: String) -> Self {
        self.operator = Some(value);
        self
    }

    pub fn target_value(
        mut self,
        value: crate::datadogV1::model::SyntheticsAssertionTargetValue,
    ) -> Self {
        self.target_value = Some(value);
        self
    }

    pub fn x_path(mut self, value: String) -> Self {
        self.x_path = Some(value);
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

impl Default for SyntheticsAssertionXPathTargetTarget {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsAssertionXPathTargetTarget {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsAssertionXPathTargetTargetVisitor;
        impl<'a> Visitor<'a> for SyntheticsAssertionXPathTargetTargetVisitor {
            type Value = SyntheticsAssertionXPathTargetTarget;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut operator: Option<String> = None;
                let mut target_value: Option<
                    crate::datadogV1::model::SyntheticsAssertionTargetValue,
                > = None;
                let mut x_path: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
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
                            if let Some(ref _target_value) = target_value {
                                match _target_value {
                                    crate::datadogV1::model::SyntheticsAssertionTargetValue::UnparsedObject(_target_value) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "xPath" => {
                            if v.is_null() {
                                continue;
                            }
                            x_path = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsAssertionXPathTargetTarget {
                    operator,
                    target_value,
                    x_path,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsAssertionXPathTargetTargetVisitor)
    }
}

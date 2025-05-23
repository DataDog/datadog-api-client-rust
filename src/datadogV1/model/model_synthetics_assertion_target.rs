// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// An assertion which uses a simple target.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsAssertionTarget {
    /// Assertion operator to apply.
    #[serde(rename = "operator")]
    pub operator: crate::datadogV1::model::SyntheticsAssertionOperator,
    /// The associated assertion property.
    #[serde(rename = "property")]
    pub property: Option<String>,
    /// Value used by the operator in assertions. Can be either a number or string.
    #[serde(rename = "target")]
    pub target: crate::datadogV1::model::SyntheticsAssertionTargetValue,
    /// Timings scope for response time assertions.
    #[serde(rename = "timingsScope")]
    pub timings_scope: Option<crate::datadogV1::model::SyntheticsAssertionTimingsScope>,
    /// Type of the assertion.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::SyntheticsAssertionType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsAssertionTarget {
    pub fn new(
        operator: crate::datadogV1::model::SyntheticsAssertionOperator,
        target: crate::datadogV1::model::SyntheticsAssertionTargetValue,
        type_: crate::datadogV1::model::SyntheticsAssertionType,
    ) -> SyntheticsAssertionTarget {
        SyntheticsAssertionTarget {
            operator,
            property: None,
            target,
            timings_scope: None,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn property(mut self, value: String) -> Self {
        self.property = Some(value);
        self
    }

    pub fn timings_scope(
        mut self,
        value: crate::datadogV1::model::SyntheticsAssertionTimingsScope,
    ) -> Self {
        self.timings_scope = Some(value);
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

impl<'de> Deserialize<'de> for SyntheticsAssertionTarget {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsAssertionTargetVisitor;
        impl<'a> Visitor<'a> for SyntheticsAssertionTargetVisitor {
            type Value = SyntheticsAssertionTarget;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut operator: Option<crate::datadogV1::model::SyntheticsAssertionOperator> =
                    None;
                let mut property: Option<String> = None;
                let mut target: Option<crate::datadogV1::model::SyntheticsAssertionTargetValue> =
                    None;
                let mut timings_scope: Option<
                    crate::datadogV1::model::SyntheticsAssertionTimingsScope,
                > = None;
                let mut type_: Option<crate::datadogV1::model::SyntheticsAssertionType> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "operator" => {
                            operator = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _operator) = operator {
                                match _operator {
                                    crate::datadogV1::model::SyntheticsAssertionOperator::UnparsedObject(_operator) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "property" => {
                            if v.is_null() {
                                continue;
                            }
                            property = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "target" => {
                            target = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _target) = target {
                                match _target {
                                    crate::datadogV1::model::SyntheticsAssertionTargetValue::UnparsedObject(_target) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "timingsScope" => {
                            if v.is_null() {
                                continue;
                            }
                            timings_scope =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _timings_scope) = timings_scope {
                                match _timings_scope {
                                    crate::datadogV1::model::SyntheticsAssertionTimingsScope::UnparsedObject(_timings_scope) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::SyntheticsAssertionType::UnparsedObject(_type_) => {
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
                let operator = operator.ok_or_else(|| M::Error::missing_field("operator"))?;
                let target = target.ok_or_else(|| M::Error::missing_field("target"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = SyntheticsAssertionTarget {
                    operator,
                    property,
                    target,
                    timings_scope,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsAssertionTargetVisitor)
    }
}

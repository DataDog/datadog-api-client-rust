// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// An assertion for the `validatesXPath` operator.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsAssertionXPathTarget {
    /// Assertion operator to apply.
    #[serde(rename = "operator")]
    pub operator: crate::datadogV1::model::SyntheticsAssertionXPathOperator,
    /// The associated assertion property.
    #[serde(rename = "property")]
    pub property: Option<String>,
    /// Composed target for `validatesXPath` operator.
    #[serde(rename = "target")]
    pub target: Option<crate::datadogV1::model::SyntheticsAssertionXPathTargetTarget>,
    /// Type of the assertion.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::SyntheticsAssertionType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsAssertionXPathTarget {
    pub fn new(
        operator: crate::datadogV1::model::SyntheticsAssertionXPathOperator,
        type_: crate::datadogV1::model::SyntheticsAssertionType,
    ) -> SyntheticsAssertionXPathTarget {
        SyntheticsAssertionXPathTarget {
            operator,
            property: None,
            target: None,
            type_,
            _unparsed: false,
        }
    }

    pub fn property(&mut self, value: String) -> &mut Self {
        self.property = Some(value);
        self
    }

    pub fn target(
        &mut self,
        value: crate::datadogV1::model::SyntheticsAssertionXPathTargetTarget,
    ) -> &mut Self {
        self.target = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for SyntheticsAssertionXPathTarget {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsAssertionXPathTargetVisitor;
        impl<'a> Visitor<'a> for SyntheticsAssertionXPathTargetVisitor {
            type Value = SyntheticsAssertionXPathTarget;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut operator: Option<
                    crate::datadogV1::model::SyntheticsAssertionXPathOperator,
                > = None;
                let mut property: Option<String> = None;
                let mut target: Option<
                    crate::datadogV1::model::SyntheticsAssertionXPathTargetTarget,
                > = None;
                let mut type_: Option<crate::datadogV1::model::SyntheticsAssertionType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "operator" => {
                            operator = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _operator) = operator {
                                match _operator {
                                    crate::datadogV1::model::SyntheticsAssertionXPathOperator::UnparsedObject(_operator) => {
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
                            if v.is_null() {
                                continue;
                            }
                            target = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        &_ => {}
                    }
                }
                let operator = operator.ok_or_else(|| M::Error::missing_field("operator"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = SyntheticsAssertionXPathTarget {
                    operator,
                    property,
                    target,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsAssertionXPathTargetVisitor)
    }
}

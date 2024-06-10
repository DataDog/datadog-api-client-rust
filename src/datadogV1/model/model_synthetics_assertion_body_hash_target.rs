// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// An assertion which targets body hash.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsAssertionBodyHashTarget {
    /// Assertion operator to apply.
    #[serde(rename = "operator")]
    pub operator: crate::datadogV1::model::SyntheticsAssertionBodyHashOperator,
    /// Value used by the operator.
    #[serde(rename = "target")]
    pub target: serde_json::Value,
    /// Type of the assertion.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::SyntheticsAssertionBodyHashType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsAssertionBodyHashTarget {
    pub fn new(
        operator: crate::datadogV1::model::SyntheticsAssertionBodyHashOperator,
        target: serde_json::Value,
        type_: crate::datadogV1::model::SyntheticsAssertionBodyHashType,
    ) -> SyntheticsAssertionBodyHashTarget {
        SyntheticsAssertionBodyHashTarget {
            operator,
            target,
            type_,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for SyntheticsAssertionBodyHashTarget {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsAssertionBodyHashTargetVisitor;
        impl<'a> Visitor<'a> for SyntheticsAssertionBodyHashTargetVisitor {
            type Value = SyntheticsAssertionBodyHashTarget;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut operator: Option<
                    crate::datadogV1::model::SyntheticsAssertionBodyHashOperator,
                > = None;
                let mut target: Option<serde_json::Value> = None;
                let mut type_: Option<crate::datadogV1::model::SyntheticsAssertionBodyHashType> =
                    None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "operator" => {
                            operator = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _operator) = operator {
                                match _operator {
                                    crate::datadogV1::model::SyntheticsAssertionBodyHashOperator::UnparsedObject(_operator) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "target" => {
                            target = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::SyntheticsAssertionBodyHashType::UnparsedObject(_type_) => {
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
                let target = target.ok_or_else(|| M::Error::missing_field("target"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = SyntheticsAssertionBodyHashTarget {
                    operator,
                    target,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsAssertionBodyHashTargetVisitor)
    }
}

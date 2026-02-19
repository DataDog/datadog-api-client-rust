// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Multi-network hop assertion for a Network Path test.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsNetworkAssertionMultiNetworkHop {
    /// Assertion operator to apply.
    #[serde(rename = "operator")]
    pub operator: crate::datadogV2::model::SyntheticsNetworkAssertionOperator,
    /// The associated assertion property.
    #[serde(rename = "property")]
    pub property: crate::datadogV2::model::SyntheticsNetworkAssertionProperty,
    /// Target value in number of hops.
    #[serde(rename = "target")]
    pub target: f64,
    /// Type of the multi-network hop assertion.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::SyntheticsNetworkAssertionMultiNetworkHopType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsNetworkAssertionMultiNetworkHop {
    pub fn new(
        operator: crate::datadogV2::model::SyntheticsNetworkAssertionOperator,
        property: crate::datadogV2::model::SyntheticsNetworkAssertionProperty,
        target: f64,
        type_: crate::datadogV2::model::SyntheticsNetworkAssertionMultiNetworkHopType,
    ) -> SyntheticsNetworkAssertionMultiNetworkHop {
        SyntheticsNetworkAssertionMultiNetworkHop {
            operator,
            property,
            target,
            type_,
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

impl<'de> Deserialize<'de> for SyntheticsNetworkAssertionMultiNetworkHop {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsNetworkAssertionMultiNetworkHopVisitor;
        impl<'a> Visitor<'a> for SyntheticsNetworkAssertionMultiNetworkHopVisitor {
            type Value = SyntheticsNetworkAssertionMultiNetworkHop;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut operator: Option<
                    crate::datadogV2::model::SyntheticsNetworkAssertionOperator,
                > = None;
                let mut property: Option<
                    crate::datadogV2::model::SyntheticsNetworkAssertionProperty,
                > = None;
                let mut target: Option<f64> = None;
                let mut type_: Option<
                    crate::datadogV2::model::SyntheticsNetworkAssertionMultiNetworkHopType,
                > = None;
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
                                    crate::datadogV2::model::SyntheticsNetworkAssertionOperator::UnparsedObject(_operator) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "property" => {
                            property = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _property) = property {
                                match _property {
                                    crate::datadogV2::model::SyntheticsNetworkAssertionProperty::UnparsedObject(_property) => {
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
                                    crate::datadogV2::model::SyntheticsNetworkAssertionMultiNetworkHopType::UnparsedObject(_type_) => {
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
                let property = property.ok_or_else(|| M::Error::missing_field("property"))?;
                let target = target.ok_or_else(|| M::Error::missing_field("target"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = SyntheticsNetworkAssertionMultiNetworkHop {
                    operator,
                    property,
                    target,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsNetworkAssertionMultiNetworkHopVisitor)
    }
}

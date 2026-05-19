// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Custom attribute values
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CustomAttributeValue {
    /// If true, value must be an array
    #[serde(rename = "is_multi")]
    pub is_multi: bool,
    /// Custom attributes type
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::CustomAttributeType,
    /// Union of supported value for a custom attribute
    #[serde(rename = "value")]
    pub value: crate::datadogV2::model::CustomAttributeValuesUnion,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CustomAttributeValue {
    pub fn new(
        is_multi: bool,
        type_: crate::datadogV2::model::CustomAttributeType,
        value: crate::datadogV2::model::CustomAttributeValuesUnion,
    ) -> CustomAttributeValue {
        CustomAttributeValue {
            is_multi,
            type_,
            value,
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

impl<'de> Deserialize<'de> for CustomAttributeValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CustomAttributeValueVisitor;
        impl<'a> Visitor<'a> for CustomAttributeValueVisitor {
            type Value = CustomAttributeValue;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut is_multi: Option<bool> = None;
                let mut type_: Option<crate::datadogV2::model::CustomAttributeType> = None;
                let mut value: Option<crate::datadogV2::model::CustomAttributeValuesUnion> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "is_multi" => {
                            is_multi = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::CustomAttributeType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "value" => {
                            value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _value) = value {
                                match _value {
                                    crate::datadogV2::model::CustomAttributeValuesUnion::UnparsedObject(_value) => {
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
                let is_multi = is_multi.ok_or_else(|| M::Error::missing_field("is_multi"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;
                let value = value.ok_or_else(|| M::Error::missing_field("value"))?;

                let content = CustomAttributeValue {
                    is_multi,
                    type_,
                    value,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CustomAttributeValueVisitor)
    }
}

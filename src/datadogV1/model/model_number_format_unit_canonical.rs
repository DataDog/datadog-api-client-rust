// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Canonical unit.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct NumberFormatUnitCanonical {
    /// The name of the unit per item.
    #[serde(rename = "per_unit_name")]
    pub per_unit_name: Option<String>,
    /// The type of unit scale.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV1::model::NumberFormatUnitScaleType>,
    /// The name of the unit.
    #[serde(rename = "unit_name")]
    pub unit_name: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl NumberFormatUnitCanonical {
    pub fn new() -> NumberFormatUnitCanonical {
        NumberFormatUnitCanonical {
            per_unit_name: None,
            type_: None,
            unit_name: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn per_unit_name(mut self, value: String) -> Self {
        self.per_unit_name = Some(value);
        self
    }

    pub fn type_(mut self, value: crate::datadogV1::model::NumberFormatUnitScaleType) -> Self {
        self.type_ = Some(value);
        self
    }

    pub fn unit_name(mut self, value: String) -> Self {
        self.unit_name = Some(value);
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

impl Default for NumberFormatUnitCanonical {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for NumberFormatUnitCanonical {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NumberFormatUnitCanonicalVisitor;
        impl<'a> Visitor<'a> for NumberFormatUnitCanonicalVisitor {
            type Value = NumberFormatUnitCanonical;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut per_unit_name: Option<String> = None;
                let mut type_: Option<crate::datadogV1::model::NumberFormatUnitScaleType> = None;
                let mut unit_name: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "per_unit_name" => {
                            if v.is_null() {
                                continue;
                            }
                            per_unit_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::NumberFormatUnitScaleType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "unit_name" => {
                            if v.is_null() {
                                continue;
                            }
                            unit_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = NumberFormatUnitCanonical {
                    per_unit_name,
                    type_,
                    unit_name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(NumberFormatUnitCanonicalVisitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A single user locator object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsMobileStepParamsElementUserLocatorValuesItems {
    /// Type of a user locator.
    #[serde(rename = "type")]
    pub type_: Option<
        crate::datadogV1::model::SyntheticsMobileStepParamsElementUserLocatorValuesItemsType,
    >,
    /// Value of a user locator.
    #[serde(rename = "value")]
    pub value: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsMobileStepParamsElementUserLocatorValuesItems {
    pub fn new() -> SyntheticsMobileStepParamsElementUserLocatorValuesItems {
        SyntheticsMobileStepParamsElementUserLocatorValuesItems {
            type_: None,
            value: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn type_(
        mut self,
        value: crate::datadogV1::model::SyntheticsMobileStepParamsElementUserLocatorValuesItemsType,
    ) -> Self {
        self.type_ = Some(value);
        self
    }

    pub fn value(mut self, value: String) -> Self {
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

impl Default for SyntheticsMobileStepParamsElementUserLocatorValuesItems {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsMobileStepParamsElementUserLocatorValuesItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsMobileStepParamsElementUserLocatorValuesItemsVisitor;
        impl<'a> Visitor<'a> for SyntheticsMobileStepParamsElementUserLocatorValuesItemsVisitor {
            type Value = SyntheticsMobileStepParamsElementUserLocatorValuesItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut type_: Option<crate::datadogV1::model::SyntheticsMobileStepParamsElementUserLocatorValuesItemsType> = None;
                let mut value: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::SyntheticsMobileStepParamsElementUserLocatorValuesItemsType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
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

                let content = SyntheticsMobileStepParamsElementUserLocatorValuesItems {
                    type_,
                    value,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsMobileStepParamsElementUserLocatorValuesItemsVisitor)
    }
}

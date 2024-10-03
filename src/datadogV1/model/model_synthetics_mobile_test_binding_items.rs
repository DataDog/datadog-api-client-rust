// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object describing the binding used for a mobile test.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsMobileTestBindingItems {
    /// List of principals for a mobile test binding.
    #[serde(rename = "principals")]
    pub principals: Option<Vec<String>>,
    /// The definition of `SyntheticsMobileTestBindingItemsRole` object.
    #[serde(rename = "role")]
    pub role: Option<crate::datadogV1::model::SyntheticsMobileTestBindingItemsRole>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsMobileTestBindingItems {
    pub fn new() -> SyntheticsMobileTestBindingItems {
        SyntheticsMobileTestBindingItems {
            principals: None,
            role: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn principals(mut self, value: Vec<String>) -> Self {
        self.principals = Some(value);
        self
    }

    pub fn role(
        mut self,
        value: crate::datadogV1::model::SyntheticsMobileTestBindingItemsRole,
    ) -> Self {
        self.role = Some(value);
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

impl Default for SyntheticsMobileTestBindingItems {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsMobileTestBindingItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsMobileTestBindingItemsVisitor;
        impl<'a> Visitor<'a> for SyntheticsMobileTestBindingItemsVisitor {
            type Value = SyntheticsMobileTestBindingItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut principals: Option<Vec<String>> = None;
                let mut role: Option<
                    crate::datadogV1::model::SyntheticsMobileTestBindingItemsRole,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "principals" => {
                            if v.is_null() {
                                continue;
                            }
                            principals = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "role" => {
                            if v.is_null() {
                                continue;
                            }
                            role = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _role) = role {
                                match _role {
                                    crate::datadogV1::model::SyntheticsMobileTestBindingItemsRole::UnparsedObject(_role) => {
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

                let content = SyntheticsMobileTestBindingItems {
                    principals,
                    role,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsMobileTestBindingItemsVisitor)
    }
}
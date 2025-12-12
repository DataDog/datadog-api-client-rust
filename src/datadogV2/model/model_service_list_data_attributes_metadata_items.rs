// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ServiceListDataAttributesMetadataItems {
    #[serde(rename = "isTraced")]
    pub is_traced: Option<bool>,
    #[serde(rename = "isUsm")]
    pub is_usm: Option<bool>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ServiceListDataAttributesMetadataItems {
    pub fn new() -> ServiceListDataAttributesMetadataItems {
        ServiceListDataAttributesMetadataItems {
            is_traced: None,
            is_usm: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn is_traced(mut self, value: bool) -> Self {
        self.is_traced = Some(value);
        self
    }

    pub fn is_usm(mut self, value: bool) -> Self {
        self.is_usm = Some(value);
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

impl Default for ServiceListDataAttributesMetadataItems {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ServiceListDataAttributesMetadataItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ServiceListDataAttributesMetadataItemsVisitor;
        impl<'a> Visitor<'a> for ServiceListDataAttributesMetadataItemsVisitor {
            type Value = ServiceListDataAttributesMetadataItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut is_traced: Option<bool> = None;
                let mut is_usm: Option<bool> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "isTraced" => {
                            if v.is_null() {
                                continue;
                            }
                            is_traced = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "isUsm" => {
                            if v.is_null() {
                                continue;
                            }
                            is_usm = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ServiceListDataAttributesMetadataItems {
                    is_traced,
                    is_usm,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ServiceListDataAttributesMetadataItemsVisitor)
    }
}

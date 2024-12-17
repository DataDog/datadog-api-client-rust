// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `ComponentGridProperties` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ComponentGridProperties {
    /// The `ComponentGridProperties` `backgroundColor`.
    #[serde(rename = "backgroundColor")]
    pub background_color: Option<String>,
    /// The `ComponentGridProperties` `children`.
    #[serde(rename = "children")]
    pub children: Option<Vec<crate::datadogV2::model::Component>>,
    /// The definition of `ComponentGridPropertiesIsVisible` object.
    #[serde(rename = "isVisible")]
    pub is_visible: Option<crate::datadogV2::model::ComponentGridPropertiesIsVisible>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ComponentGridProperties {
    pub fn new() -> ComponentGridProperties {
        ComponentGridProperties {
            background_color: None,
            children: None,
            is_visible: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn background_color(mut self, value: String) -> Self {
        self.background_color = Some(value);
        self
    }

    pub fn children(mut self, value: Vec<crate::datadogV2::model::Component>) -> Self {
        self.children = Some(value);
        self
    }

    pub fn is_visible(
        mut self,
        value: crate::datadogV2::model::ComponentGridPropertiesIsVisible,
    ) -> Self {
        self.is_visible = Some(value);
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

impl Default for ComponentGridProperties {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ComponentGridProperties {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ComponentGridPropertiesVisitor;
        impl<'a> Visitor<'a> for ComponentGridPropertiesVisitor {
            type Value = ComponentGridProperties;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut background_color: Option<String> = None;
                let mut children: Option<Vec<crate::datadogV2::model::Component>> = None;
                let mut is_visible: Option<
                    crate::datadogV2::model::ComponentGridPropertiesIsVisible,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "backgroundColor" => {
                            if v.is_null() {
                                continue;
                            }
                            background_color =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "children" => {
                            if v.is_null() {
                                continue;
                            }
                            children = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "isVisible" => {
                            if v.is_null() {
                                continue;
                            }
                            is_visible = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _is_visible) = is_visible {
                                match _is_visible {
                                    crate::datadogV2::model::ComponentGridPropertiesIsVisible::UnparsedObject(_is_visible) => {
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

                let content = ComponentGridProperties {
                    background_color,
                    children,
                    is_visible,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ComponentGridPropertiesVisitor)
    }
}

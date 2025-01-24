// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Properties of a UI component. Different component types can have their own additional unique properties. See the [components documentation](<https://docs.datadoghq.com/service_management/app_builder/components/>) for more detail on each component type and its properties.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ComponentProperties {
    /// The child components of the UI component.
    #[serde(rename = "children")]
    pub children: Option<Vec<crate::datadogV2::model::Component>>,
    /// Whether the UI component is visible. If this is a string, it must be a valid JavaScript expression that evaluates to a boolean.
    #[serde(rename = "isVisible")]
    pub is_visible: Option<crate::datadogV2::model::ComponentPropertiesIsVisible>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ComponentProperties {
    pub fn new() -> ComponentProperties {
        ComponentProperties {
            children: None,
            is_visible: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn children(mut self, value: Vec<crate::datadogV2::model::Component>) -> Self {
        self.children = Some(value);
        self
    }

    pub fn is_visible(
        mut self,
        value: crate::datadogV2::model::ComponentPropertiesIsVisible,
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

impl Default for ComponentProperties {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ComponentProperties {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ComponentPropertiesVisitor;
        impl<'a> Visitor<'a> for ComponentPropertiesVisitor {
            type Value = ComponentProperties;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut children: Option<Vec<crate::datadogV2::model::Component>> = None;
                let mut is_visible: Option<crate::datadogV2::model::ComponentPropertiesIsVisible> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
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
                                    crate::datadogV2::model::ComponentPropertiesIsVisible::UnparsedObject(_is_visible) => {
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

                let content = ComponentProperties {
                    children,
                    is_visible,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ComponentPropertiesVisitor)
    }
}

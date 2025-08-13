// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The view of the world that the map should render.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GeomapWidgetDefinitionView {
    /// A custom extent of the map defined by an array of four numbers in the order `[minLongitude, minLatitude, maxLongitude, maxLatitude]`.
    #[serde(rename = "custom_extent")]
    pub custom_extent: Option<Vec<f64>>,
    /// The ISO code of a country, sub-division, or region to focus the map on. Or `WORLD`. Mutually exclusive with `custom_extent`.
    #[serde(rename = "focus")]
    pub focus: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GeomapWidgetDefinitionView {
    pub fn new(focus: String) -> GeomapWidgetDefinitionView {
        GeomapWidgetDefinitionView {
            custom_extent: None,
            focus,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn custom_extent(mut self, value: Vec<f64>) -> Self {
        self.custom_extent = Some(value);
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

impl<'de> Deserialize<'de> for GeomapWidgetDefinitionView {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GeomapWidgetDefinitionViewVisitor;
        impl<'a> Visitor<'a> for GeomapWidgetDefinitionViewVisitor {
            type Value = GeomapWidgetDefinitionView;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut custom_extent: Option<Vec<f64>> = None;
                let mut focus: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "custom_extent" => {
                            if v.is_null() {
                                continue;
                            }
                            custom_extent =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "focus" => {
                            focus = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let focus = focus.ok_or_else(|| M::Error::missing_field("focus"))?;

                let content = GeomapWidgetDefinitionView {
                    custom_extent,
                    focus,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GeomapWidgetDefinitionViewVisitor)
    }
}

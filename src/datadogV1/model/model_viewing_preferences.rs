// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The viewing preferences for a shared dashboard.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ViewingPreferences {
    /// Whether the widgets on the shared dashboard should be displayed with high density.
    #[serde(rename = "high_density")]
    pub high_density: Option<bool>,
    /// The theme of the shared dashboard view. "system" follows your system's default viewing theme.
    #[serde(rename = "theme")]
    pub theme: Option<crate::datadogV1::model::ViewingPreferencesTheme>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ViewingPreferences {
    pub fn new() -> ViewingPreferences {
        ViewingPreferences {
            high_density: None,
            theme: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn high_density(mut self, value: bool) -> Self {
        self.high_density = Some(value);
        self
    }

    pub fn theme(mut self, value: crate::datadogV1::model::ViewingPreferencesTheme) -> Self {
        self.theme = Some(value);
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

impl Default for ViewingPreferences {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ViewingPreferences {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ViewingPreferencesVisitor;
        impl<'a> Visitor<'a> for ViewingPreferencesVisitor {
            type Value = ViewingPreferences;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut high_density: Option<bool> = None;
                let mut theme: Option<crate::datadogV1::model::ViewingPreferencesTheme> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "high_density" => {
                            if v.is_null() {
                                continue;
                            }
                            high_density =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "theme" => {
                            if v.is_null() {
                                continue;
                            }
                            theme = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _theme) = theme {
                                match _theme {
                                    crate::datadogV1::model::ViewingPreferencesTheme::UnparsedObject(_theme) => {
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

                let content = ViewingPreferences {
                    high_density,
                    theme,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ViewingPreferencesVisitor)
    }
}

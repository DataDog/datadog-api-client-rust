// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// UI configuration for rendering form fields, including widget overrides, field ordering, and themes.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FormUiDefinition {
    /// The order in which form fields are displayed.
    #[serde(rename = "ui:order")]
    pub ui_order: Option<Vec<String>>,
    /// The visual theme applied to the form.
    #[serde(rename = "ui:theme")]
    pub ui_theme: Option<crate::datadogV2::model::FormUiDefinitionUiTheme>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FormUiDefinition {
    pub fn new() -> FormUiDefinition {
        FormUiDefinition {
            ui_order: None,
            ui_theme: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn ui_order(mut self, value: Vec<String>) -> Self {
        self.ui_order = Some(value);
        self
    }

    pub fn ui_theme(mut self, value: crate::datadogV2::model::FormUiDefinitionUiTheme) -> Self {
        self.ui_theme = Some(value);
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

impl Default for FormUiDefinition {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for FormUiDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FormUiDefinitionVisitor;
        impl<'a> Visitor<'a> for FormUiDefinitionVisitor {
            type Value = FormUiDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut ui_order: Option<Vec<String>> = None;
                let mut ui_theme: Option<crate::datadogV2::model::FormUiDefinitionUiTheme> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "ui:order" => {
                            if v.is_null() {
                                continue;
                            }
                            ui_order = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ui:theme" => {
                            if v.is_null() {
                                continue;
                            }
                            ui_theme = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = FormUiDefinition {
                    ui_order,
                    ui_theme,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FormUiDefinitionVisitor)
    }
}

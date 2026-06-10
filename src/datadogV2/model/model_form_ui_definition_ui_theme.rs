// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The visual theme applied to the form.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FormUiDefinitionUiTheme {
    /// The primary color of the form theme.
    #[serde(rename = "primaryColor")]
    pub primary_color: Option<crate::datadogV2::model::FormUiDefinitionUiThemePrimaryColor>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FormUiDefinitionUiTheme {
    pub fn new() -> FormUiDefinitionUiTheme {
        FormUiDefinitionUiTheme {
            primary_color: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn primary_color(
        mut self,
        value: crate::datadogV2::model::FormUiDefinitionUiThemePrimaryColor,
    ) -> Self {
        self.primary_color = Some(value);
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

impl Default for FormUiDefinitionUiTheme {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for FormUiDefinitionUiTheme {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FormUiDefinitionUiThemeVisitor;
        impl<'a> Visitor<'a> for FormUiDefinitionUiThemeVisitor {
            type Value = FormUiDefinitionUiTheme;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut primary_color: Option<
                    crate::datadogV2::model::FormUiDefinitionUiThemePrimaryColor,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "primaryColor" => {
                            if v.is_null() {
                                continue;
                            }
                            primary_color =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _primary_color) = primary_color {
                                match _primary_color {
                                    crate::datadogV2::model::FormUiDefinitionUiThemePrimaryColor::UnparsedObject(_primary_color) => {
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

                let content = FormUiDefinitionUiTheme {
                    primary_color,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FormUiDefinitionUiThemeVisitor)
    }
}

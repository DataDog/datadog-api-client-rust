// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A template variable that viewers can modify on the shared dashboard.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SharedDashboardSelectableTemplateVariable {
    /// Whether viewers can see all tag values for the template variable and specify any value.
    #[serde(rename = "allow_any_value")]
    pub allow_any_value: bool,
    /// Default selected values for the variable.
    #[serde(rename = "default_values")]
    pub default_values: Vec<String>,
    /// Name of the template variable.
    #[serde(rename = "name")]
    pub name: String,
    /// Tag prefix for the variable.
    #[serde(rename = "prefix")]
    pub prefix: String,
    /// Type of the template variable.
    #[serde(rename = "type")]
    pub type_: String,
    /// Restricts which tag values are visible to the viewer.
    #[serde(rename = "visible_tags")]
    pub visible_tags: Vec<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SharedDashboardSelectableTemplateVariable {
    pub fn new(
        allow_any_value: bool,
        default_values: Vec<String>,
        name: String,
        prefix: String,
        type_: String,
        visible_tags: Vec<String>,
    ) -> SharedDashboardSelectableTemplateVariable {
        SharedDashboardSelectableTemplateVariable {
            allow_any_value,
            default_values,
            name,
            prefix,
            type_,
            visible_tags,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for SharedDashboardSelectableTemplateVariable {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SharedDashboardSelectableTemplateVariableVisitor;
        impl<'a> Visitor<'a> for SharedDashboardSelectableTemplateVariableVisitor {
            type Value = SharedDashboardSelectableTemplateVariable;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut allow_any_value: Option<bool> = None;
                let mut default_values: Option<Vec<String>> = None;
                let mut name: Option<String> = None;
                let mut prefix: Option<String> = None;
                let mut type_: Option<String> = None;
                let mut visible_tags: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "allow_any_value" => {
                            allow_any_value =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "default_values" => {
                            default_values =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "prefix" => {
                            prefix = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "visible_tags" => {
                            visible_tags =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let allow_any_value =
                    allow_any_value.ok_or_else(|| M::Error::missing_field("allow_any_value"))?;
                let default_values =
                    default_values.ok_or_else(|| M::Error::missing_field("default_values"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let prefix = prefix.ok_or_else(|| M::Error::missing_field("prefix"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;
                let visible_tags =
                    visible_tags.ok_or_else(|| M::Error::missing_field("visible_tags"))?;

                let content = SharedDashboardSelectableTemplateVariable {
                    allow_any_value,
                    default_values,
                    name,
                    prefix,
                    type_,
                    visible_tags,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SharedDashboardSelectableTemplateVariableVisitor)
    }
}

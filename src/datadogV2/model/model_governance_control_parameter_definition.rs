// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of a configurable parameter on a control or mitigation.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GovernanceControlParameterDefinition {
    /// The default value of the parameter. The JSON type depends on the parameter's `type`.
    #[serde(rename = "default_value")]
    pub default_value: serde_json::Value,
    /// A human-readable description of the parameter.
    #[serde(rename = "description")]
    pub description: String,
    /// The human-readable name of the parameter.
    #[serde(rename = "display_name")]
    pub display_name: String,
    /// Whether the parameter is hidden from the UI.
    #[serde(rename = "hidden")]
    pub hidden: bool,
    /// The machine-readable name of the parameter.
    #[serde(rename = "name")]
    pub name: String,
    /// Whether the parameter must be provided.
    #[serde(rename = "required")]
    pub required: bool,
    /// The supported values for an enumerated parameter.
    #[serde(rename = "supported_values")]
    pub supported_values: Vec<crate::datadogV2::model::GovernanceControlSupportedValue>,
    /// The type of the parameter, such as `integer`, `string`, `boolean`, `enum`, or `pattern_list`.
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GovernanceControlParameterDefinition {
    pub fn new(
        default_value: serde_json::Value,
        description: String,
        display_name: String,
        hidden: bool,
        name: String,
        required: bool,
        supported_values: Vec<crate::datadogV2::model::GovernanceControlSupportedValue>,
        type_: String,
    ) -> GovernanceControlParameterDefinition {
        GovernanceControlParameterDefinition {
            default_value,
            description,
            display_name,
            hidden,
            name,
            required,
            supported_values,
            type_,
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

impl<'de> Deserialize<'de> for GovernanceControlParameterDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GovernanceControlParameterDefinitionVisitor;
        impl<'a> Visitor<'a> for GovernanceControlParameterDefinitionVisitor {
            type Value = GovernanceControlParameterDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut default_value: Option<serde_json::Value> = None;
                let mut description: Option<String> = None;
                let mut display_name: Option<String> = None;
                let mut hidden: Option<bool> = None;
                let mut name: Option<String> = None;
                let mut required: Option<bool> = None;
                let mut supported_values: Option<
                    Vec<crate::datadogV2::model::GovernanceControlSupportedValue>,
                > = None;
                let mut type_: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "default_value" => {
                            default_value =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "display_name" => {
                            display_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "hidden" => {
                            hidden = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "required" => {
                            required = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "supported_values" => {
                            supported_values =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let default_value =
                    default_value.ok_or_else(|| M::Error::missing_field("default_value"))?;
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
                let display_name =
                    display_name.ok_or_else(|| M::Error::missing_field("display_name"))?;
                let hidden = hidden.ok_or_else(|| M::Error::missing_field("hidden"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let required = required.ok_or_else(|| M::Error::missing_field("required"))?;
                let supported_values =
                    supported_values.ok_or_else(|| M::Error::missing_field("supported_values"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = GovernanceControlParameterDefinition {
                    default_value,
                    description,
                    display_name,
                    hidden,
                    name,
                    required,
                    supported_values,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GovernanceControlParameterDefinitionVisitor)
    }
}

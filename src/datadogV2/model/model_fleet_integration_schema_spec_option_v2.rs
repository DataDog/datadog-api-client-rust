// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A single configuration option within an integration's configuration file.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FleetIntegrationSchemaSpecOptionV2 {
    /// Deprecation information for a configuration option. Currently carries no fields and is always emitted as an empty object or `null`.
    #[serialize_always]
    #[serde(rename = "deprecation")]
    pub deprecation: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// A human-readable description of the option.
    #[serde(rename = "description")]
    pub description: String,
    /// The display order priority of the option relative to other options.
    #[serde(rename = "display_priority")]
    pub display_priority: i64,
    /// Whether the option is enabled by default.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// An example value for the option. Can be any JSON type. Absent from the response when not set.
    #[serde(rename = "example")]
    pub example: Option<serde_json::Value>,
    /// Whether the option is hidden from the default configuration UI.
    #[serde(rename = "hidden")]
    pub hidden: bool,
    /// Metadata tags associated with the option. Returned as an empty array when the option has no tags.
    #[serde(rename = "metadata_tags")]
    pub metadata_tags: Vec<String>,
    /// Whether the option accepts multiple values.
    #[serde(rename = "multiple")]
    pub multiple: bool,
    /// Whether multiple instances of this option are defined in the configuration file.
    #[serde(rename = "multiple_instances_defined")]
    pub multiple_instances_defined: bool,
    /// The option name.
    #[serde(rename = "name")]
    pub name: String,
    /// Nested options. Absent from the response when the option has no nested options.
    #[serde(rename = "options")]
    pub options: Option<Vec<crate::datadogV2::model::FleetIntegrationSchemaSpecOptionV2>>,
    /// A prefill value for the option. Can be any JSON type. Absent from the response when not set.
    #[serde(rename = "prefill")]
    pub prefill: Option<serde_json::Value>,
    /// Whether the option is required.
    #[serde(rename = "required")]
    pub required: bool,
    /// Whether the option is a secret that should be masked. Absent from the response when not set, distinct from being explicitly set to `false`.
    #[serde(rename = "secret")]
    pub secret: Option<bool>,
    /// A JSON Schema-like specification for a configuration value.
    ///
    /// Object-typed values always include a `properties` array, even when empty.
    /// Non-object-typed values never include `properties`. Throughout this schema,
    /// an empty array is meaningfully different from an absent field.
    ///
    /// Three further JSON Schema keywords can appear directly on this object but are
    /// not listed among its properties below to avoid clashing with this document's
    /// own schema composition keywords: `enum` (an array of allowed values, present
    /// only when there are enum constraints), `required` (an array of required
    /// property names, present only when `type` is `object`), and `oneOf` (an array
    /// of exclusive alternative value specifications this value can match, present
    /// only when there are alternatives).
    #[serde(rename = "value")]
    pub value: Option<crate::datadogV2::model::FleetIntegrationSchemaSpecValueV2>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FleetIntegrationSchemaSpecOptionV2 {
    pub fn new(
        deprecation: Option<std::collections::BTreeMap<String, serde_json::Value>>,
        description: String,
        display_priority: i64,
        enabled: bool,
        hidden: bool,
        metadata_tags: Vec<String>,
        multiple: bool,
        multiple_instances_defined: bool,
        name: String,
        required: bool,
    ) -> FleetIntegrationSchemaSpecOptionV2 {
        FleetIntegrationSchemaSpecOptionV2 {
            deprecation,
            description,
            display_priority,
            enabled,
            example: None,
            hidden,
            metadata_tags,
            multiple,
            multiple_instances_defined,
            name,
            options: None,
            prefill: None,
            required,
            secret: None,
            value: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn example(mut self, value: serde_json::Value) -> Self {
        self.example = Some(value);
        self
    }

    pub fn options(
        mut self,
        value: Vec<crate::datadogV2::model::FleetIntegrationSchemaSpecOptionV2>,
    ) -> Self {
        self.options = Some(value);
        self
    }

    pub fn prefill(mut self, value: serde_json::Value) -> Self {
        self.prefill = Some(value);
        self
    }

    pub fn secret(mut self, value: bool) -> Self {
        self.secret = Some(value);
        self
    }

    pub fn value(
        mut self,
        value: crate::datadogV2::model::FleetIntegrationSchemaSpecValueV2,
    ) -> Self {
        self.value = Some(value);
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

impl<'de> Deserialize<'de> for FleetIntegrationSchemaSpecOptionV2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FleetIntegrationSchemaSpecOptionV2Visitor;
        impl<'a> Visitor<'a> for FleetIntegrationSchemaSpecOptionV2Visitor {
            type Value = FleetIntegrationSchemaSpecOptionV2;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut deprecation: Option<
                    Option<std::collections::BTreeMap<String, serde_json::Value>>,
                > = None;
                let mut description: Option<String> = None;
                let mut display_priority: Option<i64> = None;
                let mut enabled: Option<bool> = None;
                let mut example: Option<serde_json::Value> = None;
                let mut hidden: Option<bool> = None;
                let mut metadata_tags: Option<Vec<String>> = None;
                let mut multiple: Option<bool> = None;
                let mut multiple_instances_defined: Option<bool> = None;
                let mut name: Option<String> = None;
                let mut options: Option<
                    Vec<crate::datadogV2::model::FleetIntegrationSchemaSpecOptionV2>,
                > = None;
                let mut prefill: Option<serde_json::Value> = None;
                let mut required: Option<bool> = None;
                let mut secret: Option<bool> = None;
                let mut value: Option<crate::datadogV2::model::FleetIntegrationSchemaSpecValueV2> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "deprecation" => {
                            deprecation =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "display_priority" => {
                            display_priority =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enabled" => {
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "example" => {
                            if v.is_null() {
                                continue;
                            }
                            example = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "hidden" => {
                            hidden = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metadata_tags" => {
                            metadata_tags =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "multiple" => {
                            multiple = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "multiple_instances_defined" => {
                            multiple_instances_defined =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "options" => {
                            if v.is_null() {
                                continue;
                            }
                            options = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "prefill" => {
                            if v.is_null() {
                                continue;
                            }
                            prefill = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "required" => {
                            required = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "secret" => {
                            if v.is_null() {
                                continue;
                            }
                            secret = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "value" => {
                            if v.is_null() {
                                continue;
                            }
                            value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let deprecation =
                    deprecation.ok_or_else(|| M::Error::missing_field("deprecation"))?;
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
                let display_priority =
                    display_priority.ok_or_else(|| M::Error::missing_field("display_priority"))?;
                let enabled = enabled.ok_or_else(|| M::Error::missing_field("enabled"))?;
                let hidden = hidden.ok_or_else(|| M::Error::missing_field("hidden"))?;
                let metadata_tags =
                    metadata_tags.ok_or_else(|| M::Error::missing_field("metadata_tags"))?;
                let multiple = multiple.ok_or_else(|| M::Error::missing_field("multiple"))?;
                let multiple_instances_defined = multiple_instances_defined
                    .ok_or_else(|| M::Error::missing_field("multiple_instances_defined"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let required = required.ok_or_else(|| M::Error::missing_field("required"))?;

                let content = FleetIntegrationSchemaSpecOptionV2 {
                    deprecation,
                    description,
                    display_priority,
                    enabled,
                    example,
                    hidden,
                    metadata_tags,
                    multiple,
                    multiple_instances_defined,
                    name,
                    options,
                    prefill,
                    required,
                    secret,
                    value,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FleetIntegrationSchemaSpecOptionV2Visitor)
    }
}

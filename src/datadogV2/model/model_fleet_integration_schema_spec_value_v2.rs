// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

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
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FleetIntegrationSchemaSpecValueV2 {
    /// Whether, or which, additional properties are allowed on the object. Can be a boolean or a nested schema. Present only when `type` is `object`.
    #[serde(rename = "additionalProperties")]
    pub additional_properties: Option<serde_json::Value>,
    /// Alternative value specifications this value can match. Absent when none apply.
    #[serde(rename = "anyOf")]
    pub any_of: Option<Vec<crate::datadogV2::model::FleetIntegrationSchemaSpecValueV2>>,
    /// The default value. Can be any JSON type. Absent when not set.
    #[serde(rename = "default")]
    pub default: Option<serde_json::Value>,
    /// A human-readable description of the value. Absent when not set.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// A legacy, display-formatted representation of the default value. Can be any JSON type. Absent when not set.
    #[serde(rename = "display_default")]
    pub display_default: Option<serde_json::Value>,
    /// An example value. Can be any JSON type. Absent when not set.
    #[serde(rename = "example")]
    pub example: Option<serde_json::Value>,
    /// The maximum allowed numeric value, exclusive. Absent when not set.
    #[serde(rename = "exclusiveMaximum")]
    pub exclusive_maximum: Option<f64>,
    /// The minimum allowed numeric value, exclusive. Absent when not set.
    #[serde(rename = "exclusiveMinimum")]
    pub exclusive_minimum: Option<f64>,
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
    #[serde(rename = "items")]
    pub items: Option<crate::datadogV2::model::FleetIntegrationSchemaSpecValueV2>,
    /// The maximum allowed string length. Absent when not set.
    #[serde(rename = "maxLength")]
    pub max_length: Option<i64>,
    /// The maximum allowed numeric value, inclusive. Absent when not set.
    #[serde(rename = "maximum")]
    pub maximum: Option<f64>,
    /// The minimum allowed string length. Absent when not set.
    #[serde(rename = "minLength")]
    pub min_length: Option<i64>,
    /// The minimum allowed numeric value, inclusive. Absent when not set.
    #[serde(rename = "minimum")]
    pub minimum: Option<f64>,
    /// A regular expression the string value must match. Absent when not set.
    #[serde(rename = "pattern")]
    pub pattern: Option<String>,
    /// The object's declared properties. Present when `type` is `object`, including as an empty array when the object declares no properties. Absent for non-object types.
    #[serde(rename = "properties")]
    pub properties: Option<Vec<crate::datadogV2::model::FleetIntegrationSchemaSpecPropertyV2>>,
    /// Whether the value is a secret that should be masked. Absent when not set.
    #[serde(rename = "secret")]
    pub secret: Option<bool>,
    /// The JSON Schema type of the value, such as `string` or `object`. Absent when not set.
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FleetIntegrationSchemaSpecValueV2 {
    pub fn new() -> FleetIntegrationSchemaSpecValueV2 {
        FleetIntegrationSchemaSpecValueV2 {
            additional_properties: None,
            any_of: None,
            default: None,
            description: None,
            display_default: None,
            example: None,
            exclusive_maximum: None,
            exclusive_minimum: None,
            items: None,
            max_length: None,
            maximum: None,
            min_length: None,
            minimum: None,
            pattern: None,
            properties: None,
            secret: None,
            type_: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(mut self, value: serde_json::Value) -> Self {
        self.additional_properties = Some(value);
        self
    }

    pub fn any_of(
        mut self,
        value: Vec<crate::datadogV2::model::FleetIntegrationSchemaSpecValueV2>,
    ) -> Self {
        self.any_of = Some(value);
        self
    }

    pub fn default(mut self, value: serde_json::Value) -> Self {
        self.default = Some(value);
        self
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn display_default(mut self, value: serde_json::Value) -> Self {
        self.display_default = Some(value);
        self
    }

    pub fn example(mut self, value: serde_json::Value) -> Self {
        self.example = Some(value);
        self
    }

    pub fn exclusive_maximum(mut self, value: f64) -> Self {
        self.exclusive_maximum = Some(value);
        self
    }

    pub fn exclusive_minimum(mut self, value: f64) -> Self {
        self.exclusive_minimum = Some(value);
        self
    }

    pub fn items(
        mut self,
        value: crate::datadogV2::model::FleetIntegrationSchemaSpecValueV2,
    ) -> Self {
        self.items = Some(value);
        self
    }

    pub fn max_length(mut self, value: i64) -> Self {
        self.max_length = Some(value);
        self
    }

    pub fn maximum(mut self, value: f64) -> Self {
        self.maximum = Some(value);
        self
    }

    pub fn min_length(mut self, value: i64) -> Self {
        self.min_length = Some(value);
        self
    }

    pub fn minimum(mut self, value: f64) -> Self {
        self.minimum = Some(value);
        self
    }

    pub fn pattern(mut self, value: String) -> Self {
        self.pattern = Some(value);
        self
    }

    pub fn properties(
        mut self,
        value: Vec<crate::datadogV2::model::FleetIntegrationSchemaSpecPropertyV2>,
    ) -> Self {
        self.properties = Some(value);
        self
    }

    pub fn secret(mut self, value: bool) -> Self {
        self.secret = Some(value);
        self
    }

    pub fn type_(mut self, value: String) -> Self {
        self.type_ = Some(value);
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

impl Default for FleetIntegrationSchemaSpecValueV2 {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for FleetIntegrationSchemaSpecValueV2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FleetIntegrationSchemaSpecValueV2Visitor;
        impl<'a> Visitor<'a> for FleetIntegrationSchemaSpecValueV2Visitor {
            type Value = FleetIntegrationSchemaSpecValueV2;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut additional_properties: Option<serde_json::Value> = None;
                let mut any_of: Option<
                    Vec<crate::datadogV2::model::FleetIntegrationSchemaSpecValueV2>,
                > = None;
                let mut default: Option<serde_json::Value> = None;
                let mut description: Option<String> = None;
                let mut display_default: Option<serde_json::Value> = None;
                let mut example: Option<serde_json::Value> = None;
                let mut exclusive_maximum: Option<f64> = None;
                let mut exclusive_minimum: Option<f64> = None;
                let mut items: Option<crate::datadogV2::model::FleetIntegrationSchemaSpecValueV2> =
                    None;
                let mut max_length: Option<i64> = None;
                let mut maximum: Option<f64> = None;
                let mut min_length: Option<i64> = None;
                let mut minimum: Option<f64> = None;
                let mut pattern: Option<String> = None;
                let mut properties: Option<
                    Vec<crate::datadogV2::model::FleetIntegrationSchemaSpecPropertyV2>,
                > = None;
                let mut secret: Option<bool> = None;
                let mut type_: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "additionalProperties" => {
                            if v.is_null() {
                                continue;
                            }
                            additional_properties =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "anyOf" => {
                            if v.is_null() {
                                continue;
                            }
                            any_of = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "default" => {
                            if v.is_null() {
                                continue;
                            }
                            default = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "display_default" => {
                            if v.is_null() {
                                continue;
                            }
                            display_default =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "example" => {
                            if v.is_null() {
                                continue;
                            }
                            example = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "exclusiveMaximum" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            exclusive_maximum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "exclusiveMinimum" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            exclusive_minimum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "items" => {
                            if v.is_null() {
                                continue;
                            }
                            items = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "maxLength" => {
                            if v.is_null() {
                                continue;
                            }
                            max_length = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "maximum" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            maximum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "minLength" => {
                            if v.is_null() {
                                continue;
                            }
                            min_length = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "minimum" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            minimum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "pattern" => {
                            if v.is_null() {
                                continue;
                            }
                            pattern = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "properties" => {
                            if v.is_null() {
                                continue;
                            }
                            properties = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "secret" => {
                            if v.is_null() {
                                continue;
                            }
                            secret = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = FleetIntegrationSchemaSpecValueV2 {
                    additional_properties,
                    any_of,
                    default,
                    description,
                    display_default,
                    example,
                    exclusive_maximum,
                    exclusive_minimum,
                    items,
                    max_length,
                    maximum,
                    min_length,
                    minimum,
                    pattern,
                    properties,
                    secret,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FleetIntegrationSchemaSpecValueV2Visitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A property of an object-typed configuration value. A `oneOf` keyword (an array of exclusive alternative value specifications this property can match) can appear directly on this object when alternatives apply.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FleetIntegrationSchemaSpecPropertyV2 {
    /// Whether, or which, additional properties are allowed on the object. Can be a boolean or a nested schema. Present only when `type` is `object`.
    #[serde(rename = "additionalProperties")]
    pub additional_properties: Option<serde_json::Value>,
    /// Alternative value specifications this property can match. Absent when none apply.
    #[serde(rename = "anyOf")]
    pub any_of: Option<Vec<crate::datadogV2::model::FleetIntegrationSchemaSpecValueV2>>,
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
    /// The property name.
    #[serde(rename = "name")]
    pub name: String,
    /// Nested properties. Present only when `type` is `object` and the object declares properties.
    #[serde(rename = "properties")]
    pub properties: Option<Vec<crate::datadogV2::model::FleetIntegrationSchemaSpecPropertyV2>>,
    /// The JSON Schema type of the property, such as `string` or `boolean`. Absent when not set.
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FleetIntegrationSchemaSpecPropertyV2 {
    pub fn new(name: String) -> FleetIntegrationSchemaSpecPropertyV2 {
        FleetIntegrationSchemaSpecPropertyV2 {
            additional_properties: None,
            any_of: None,
            items: None,
            name,
            properties: None,
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

    pub fn items(
        mut self,
        value: crate::datadogV2::model::FleetIntegrationSchemaSpecValueV2,
    ) -> Self {
        self.items = Some(value);
        self
    }

    pub fn properties(
        mut self,
        value: Vec<crate::datadogV2::model::FleetIntegrationSchemaSpecPropertyV2>,
    ) -> Self {
        self.properties = Some(value);
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

impl<'de> Deserialize<'de> for FleetIntegrationSchemaSpecPropertyV2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FleetIntegrationSchemaSpecPropertyV2Visitor;
        impl<'a> Visitor<'a> for FleetIntegrationSchemaSpecPropertyV2Visitor {
            type Value = FleetIntegrationSchemaSpecPropertyV2;

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
                let mut items: Option<crate::datadogV2::model::FleetIntegrationSchemaSpecValueV2> =
                    None;
                let mut name: Option<String> = None;
                let mut properties: Option<
                    Vec<crate::datadogV2::model::FleetIntegrationSchemaSpecPropertyV2>,
                > = None;
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
                        "items" => {
                            if v.is_null() {
                                continue;
                            }
                            items = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "properties" => {
                            if v.is_null() {
                                continue;
                            }
                            properties = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = FleetIntegrationSchemaSpecPropertyV2 {
                    additional_properties,
                    any_of,
                    items,
                    name,
                    properties,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FleetIntegrationSchemaSpecPropertyV2Visitor)
    }
}

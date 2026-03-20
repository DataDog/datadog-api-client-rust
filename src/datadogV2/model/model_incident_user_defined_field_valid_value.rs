// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A valid value for an incident user-defined field.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentUserDefinedFieldValidValue {
    /// A detailed description of the valid value.
    #[serde(rename = "description")]
    pub description: String,
    /// The human-readable display name for this value.
    #[serde(rename = "display_name")]
    pub display_name: String,
    /// A short description of the valid value.
    #[serde(rename = "short_description")]
    pub short_description: Option<String>,
    /// The machine-readable value stored when this option is selected.
    #[serde(rename = "value")]
    pub value: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentUserDefinedFieldValidValue {
    pub fn new(
        description: String,
        display_name: String,
        value: String,
    ) -> IncidentUserDefinedFieldValidValue {
        IncidentUserDefinedFieldValidValue {
            description,
            display_name,
            short_description: None,
            value,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn short_description(mut self, value: String) -> Self {
        self.short_description = Some(value);
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

impl<'de> Deserialize<'de> for IncidentUserDefinedFieldValidValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentUserDefinedFieldValidValueVisitor;
        impl<'a> Visitor<'a> for IncidentUserDefinedFieldValidValueVisitor {
            type Value = IncidentUserDefinedFieldValidValue;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut description: Option<String> = None;
                let mut display_name: Option<String> = None;
                let mut short_description: Option<String> = None;
                let mut value: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "display_name" => {
                            display_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "short_description" => {
                            if v.is_null() {
                                continue;
                            }
                            short_description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "value" => {
                            value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
                let display_name =
                    display_name.ok_or_else(|| M::Error::missing_field("display_name"))?;
                let value = value.ok_or_else(|| M::Error::missing_field("value"))?;

                let content = IncidentUserDefinedFieldValidValue {
                    description,
                    display_name,
                    short_description,
                    value,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentUserDefinedFieldValidValueVisitor)
    }
}

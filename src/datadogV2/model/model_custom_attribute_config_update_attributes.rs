// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes that can be updated on a custom attribute configuration. All fields are optional; only provided fields are changed.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CustomAttributeConfigUpdateAttributes {
    /// A description explaining the purpose and expected values for this custom attribute.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// The human-readable label shown in the Case Management UI for this custom attribute.
    #[serde(rename = "display_name")]
    pub display_name: Option<String>,
    /// An external field identifier to auto-populate this attribute from (used for integrations with external systems).
    #[serde(rename = "map_from")]
    pub map_from: Option<String>,
    /// The data type of the custom attribute, which determines the allowed values and UI input control.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::CustomAttributeType>,
    /// Type-specific configuration for the custom attribute. For SELECT-type attributes, this contains the list of allowed options.
    #[serde(rename = "type_data")]
    pub type_data: Option<crate::datadogV2::model::CustomAttributeTypeData>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CustomAttributeConfigUpdateAttributes {
    pub fn new() -> CustomAttributeConfigUpdateAttributes {
        CustomAttributeConfigUpdateAttributes {
            description: None,
            display_name: None,
            map_from: None,
            type_: None,
            type_data: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn display_name(mut self, value: String) -> Self {
        self.display_name = Some(value);
        self
    }

    pub fn map_from(mut self, value: String) -> Self {
        self.map_from = Some(value);
        self
    }

    pub fn type_(mut self, value: crate::datadogV2::model::CustomAttributeType) -> Self {
        self.type_ = Some(value);
        self
    }

    pub fn type_data(mut self, value: crate::datadogV2::model::CustomAttributeTypeData) -> Self {
        self.type_data = Some(value);
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

impl Default for CustomAttributeConfigUpdateAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CustomAttributeConfigUpdateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CustomAttributeConfigUpdateAttributesVisitor;
        impl<'a> Visitor<'a> for CustomAttributeConfigUpdateAttributesVisitor {
            type Value = CustomAttributeConfigUpdateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut description: Option<String> = None;
                let mut display_name: Option<String> = None;
                let mut map_from: Option<String> = None;
                let mut type_: Option<crate::datadogV2::model::CustomAttributeType> = None;
                let mut type_data: Option<crate::datadogV2::model::CustomAttributeTypeData> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "display_name" => {
                            if v.is_null() {
                                continue;
                            }
                            display_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "map_from" => {
                            if v.is_null() {
                                continue;
                            }
                            map_from = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::CustomAttributeType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "type_data" => {
                            if v.is_null() {
                                continue;
                            }
                            type_data = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = CustomAttributeConfigUpdateAttributes {
                    description,
                    display_name,
                    map_from,
                    type_,
                    type_data,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CustomAttributeConfigUpdateAttributesVisitor)
    }
}

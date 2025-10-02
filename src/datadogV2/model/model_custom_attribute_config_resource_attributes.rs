// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Custom attribute resource attributes
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CustomAttributeConfigResourceAttributes {
    /// Custom attribute config identifier.
    #[serde(rename = "case_type_id")]
    pub case_type_id: String,
    /// Custom attribute description.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Custom attribute name.
    #[serde(rename = "display_name")]
    pub display_name: String,
    /// Whether multiple values can be set
    #[serde(rename = "is_multi")]
    pub is_multi: bool,
    /// Custom attribute key. This will be the value use to search on this custom attribute
    #[serde(rename = "key")]
    pub key: String,
    /// Custom attributes type
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::CustomAttributeType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CustomAttributeConfigResourceAttributes {
    pub fn new(
        case_type_id: String,
        display_name: String,
        is_multi: bool,
        key: String,
        type_: crate::datadogV2::model::CustomAttributeType,
    ) -> CustomAttributeConfigResourceAttributes {
        CustomAttributeConfigResourceAttributes {
            case_type_id,
            description: None,
            display_name,
            is_multi,
            key,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
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

impl<'de> Deserialize<'de> for CustomAttributeConfigResourceAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CustomAttributeConfigResourceAttributesVisitor;
        impl<'a> Visitor<'a> for CustomAttributeConfigResourceAttributesVisitor {
            type Value = CustomAttributeConfigResourceAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut case_type_id: Option<String> = None;
                let mut description: Option<String> = None;
                let mut display_name: Option<String> = None;
                let mut is_multi: Option<bool> = None;
                let mut key: Option<String> = None;
                let mut type_: Option<crate::datadogV2::model::CustomAttributeType> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "case_type_id" => {
                            case_type_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "display_name" => {
                            display_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_multi" => {
                            is_multi = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "key" => {
                            key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
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
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let case_type_id =
                    case_type_id.ok_or_else(|| M::Error::missing_field("case_type_id"))?;
                let display_name =
                    display_name.ok_or_else(|| M::Error::missing_field("display_name"))?;
                let is_multi = is_multi.ok_or_else(|| M::Error::missing_field("is_multi"))?;
                let key = key.ok_or_else(|| M::Error::missing_field("key"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = CustomAttributeConfigResourceAttributes {
                    case_type_id,
                    description,
                    display_name,
                    is_multi,
                    key,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CustomAttributeConfigResourceAttributesVisitor)
    }
}

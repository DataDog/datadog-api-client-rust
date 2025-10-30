// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UpdateConnectionRequestDataAttributesFieldsToUpdateItems {
    #[serde(rename = "field_id")]
    pub field_id: String,
    #[serde(rename = "updated_description")]
    pub updated_description: Option<String>,
    #[serde(rename = "updated_display_name")]
    pub updated_display_name: Option<String>,
    #[serde(rename = "updated_field_id")]
    pub updated_field_id: Option<String>,
    #[serde(rename = "updated_groups")]
    pub updated_groups: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UpdateConnectionRequestDataAttributesFieldsToUpdateItems {
    pub fn new(field_id: String) -> UpdateConnectionRequestDataAttributesFieldsToUpdateItems {
        UpdateConnectionRequestDataAttributesFieldsToUpdateItems {
            field_id,
            updated_description: None,
            updated_display_name: None,
            updated_field_id: None,
            updated_groups: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn updated_description(mut self, value: String) -> Self {
        self.updated_description = Some(value);
        self
    }

    pub fn updated_display_name(mut self, value: String) -> Self {
        self.updated_display_name = Some(value);
        self
    }

    pub fn updated_field_id(mut self, value: String) -> Self {
        self.updated_field_id = Some(value);
        self
    }

    pub fn updated_groups(mut self, value: Vec<String>) -> Self {
        self.updated_groups = Some(value);
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

impl<'de> Deserialize<'de> for UpdateConnectionRequestDataAttributesFieldsToUpdateItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UpdateConnectionRequestDataAttributesFieldsToUpdateItemsVisitor;
        impl<'a> Visitor<'a> for UpdateConnectionRequestDataAttributesFieldsToUpdateItemsVisitor {
            type Value = UpdateConnectionRequestDataAttributesFieldsToUpdateItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut field_id: Option<String> = None;
                let mut updated_description: Option<String> = None;
                let mut updated_display_name: Option<String> = None;
                let mut updated_field_id: Option<String> = None;
                let mut updated_groups: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "field_id" => {
                            field_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_description" => {
                            if v.is_null() {
                                continue;
                            }
                            updated_description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_display_name" => {
                            if v.is_null() {
                                continue;
                            }
                            updated_display_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_field_id" => {
                            if v.is_null() {
                                continue;
                            }
                            updated_field_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_groups" => {
                            if v.is_null() {
                                continue;
                            }
                            updated_groups =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let field_id = field_id.ok_or_else(|| M::Error::missing_field("field_id"))?;

                let content = UpdateConnectionRequestDataAttributesFieldsToUpdateItems {
                    field_id,
                    updated_description,
                    updated_display_name,
                    updated_field_id,
                    updated_groups,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(UpdateConnectionRequestDataAttributesFieldsToUpdateItemsVisitor)
    }
}

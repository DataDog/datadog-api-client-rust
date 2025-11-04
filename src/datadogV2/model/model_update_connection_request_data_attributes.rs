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
pub struct UpdateConnectionRequestDataAttributes {
    #[serde(rename = "fields_to_add")]
    pub fields_to_add:
        Option<Vec<crate::datadogV2::model::CreateConnectionRequestDataAttributesFieldsItems>>,
    #[serde(rename = "fields_to_delete")]
    pub fields_to_delete: Option<Vec<String>>,
    #[serde(rename = "fields_to_update")]
    pub fields_to_update: Option<
        Vec<crate::datadogV2::model::UpdateConnectionRequestDataAttributesFieldsToUpdateItems>,
    >,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UpdateConnectionRequestDataAttributes {
    pub fn new() -> UpdateConnectionRequestDataAttributes {
        UpdateConnectionRequestDataAttributes {
            fields_to_add: None,
            fields_to_delete: None,
            fields_to_update: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn fields_to_add(
        mut self,
        value: Vec<crate::datadogV2::model::CreateConnectionRequestDataAttributesFieldsItems>,
    ) -> Self {
        self.fields_to_add = Some(value);
        self
    }

    pub fn fields_to_delete(mut self, value: Vec<String>) -> Self {
        self.fields_to_delete = Some(value);
        self
    }

    pub fn fields_to_update(
        mut self,
        value: Vec<
            crate::datadogV2::model::UpdateConnectionRequestDataAttributesFieldsToUpdateItems,
        >,
    ) -> Self {
        self.fields_to_update = Some(value);
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

impl Default for UpdateConnectionRequestDataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for UpdateConnectionRequestDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UpdateConnectionRequestDataAttributesVisitor;
        impl<'a> Visitor<'a> for UpdateConnectionRequestDataAttributesVisitor {
            type Value = UpdateConnectionRequestDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut fields_to_add: Option<
                    Vec<crate::datadogV2::model::CreateConnectionRequestDataAttributesFieldsItems>,
                > = None;
                let mut fields_to_delete: Option<Vec<String>> = None;
                let mut fields_to_update: Option<Vec<crate::datadogV2::model::UpdateConnectionRequestDataAttributesFieldsToUpdateItems>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "fields_to_add" => {
                            if v.is_null() {
                                continue;
                            }
                            fields_to_add =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "fields_to_delete" => {
                            if v.is_null() {
                                continue;
                            }
                            fields_to_delete =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "fields_to_update" => {
                            if v.is_null() {
                                continue;
                            }
                            fields_to_update =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = UpdateConnectionRequestDataAttributes {
                    fields_to_add,
                    fields_to_delete,
                    fields_to_update,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UpdateConnectionRequestDataAttributesVisitor)
    }
}

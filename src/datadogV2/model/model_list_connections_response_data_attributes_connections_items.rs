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
pub struct ListConnectionsResponseDataAttributesConnectionsItems {
    #[serde(rename = "created_at")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "created_by")]
    pub created_by: Option<String>,
    #[serde(rename = "fields")]
    pub fields:
        Option<Vec<crate::datadogV2::model::CreateConnectionRequestDataAttributesFieldsItems>>,
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[serde(rename = "join")]
    pub join:
        Option<crate::datadogV2::model::ListConnectionsResponseDataAttributesConnectionsItemsJoin>,
    #[serde(rename = "metadata")]
    pub metadata: Option<std::collections::BTreeMap<String, String>>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(rename = "updated_at")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "updated_by")]
    pub updated_by: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ListConnectionsResponseDataAttributesConnectionsItems {
    pub fn new() -> ListConnectionsResponseDataAttributesConnectionsItems {
        ListConnectionsResponseDataAttributesConnectionsItems {
            created_at: None,
            created_by: None,
            fields: None,
            id: None,
            join: None,
            metadata: None,
            type_: None,
            updated_at: None,
            updated_by: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn created_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn created_by(mut self, value: String) -> Self {
        self.created_by = Some(value);
        self
    }

    pub fn fields(
        mut self,
        value: Vec<crate::datadogV2::model::CreateConnectionRequestDataAttributesFieldsItems>,
    ) -> Self {
        self.fields = Some(value);
        self
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }

    pub fn join(
        mut self,
        value: crate::datadogV2::model::ListConnectionsResponseDataAttributesConnectionsItemsJoin,
    ) -> Self {
        self.join = Some(value);
        self
    }

    pub fn metadata(mut self, value: std::collections::BTreeMap<String, String>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn type_(mut self, value: String) -> Self {
        self.type_ = Some(value);
        self
    }

    pub fn updated_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn updated_by(mut self, value: String) -> Self {
        self.updated_by = Some(value);
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

impl Default for ListConnectionsResponseDataAttributesConnectionsItems {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ListConnectionsResponseDataAttributesConnectionsItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ListConnectionsResponseDataAttributesConnectionsItemsVisitor;
        impl<'a> Visitor<'a> for ListConnectionsResponseDataAttributesConnectionsItemsVisitor {
            type Value = ListConnectionsResponseDataAttributesConnectionsItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut created_by: Option<String> = None;
                let mut fields: Option<
                    Vec<crate::datadogV2::model::CreateConnectionRequestDataAttributesFieldsItems>,
                > = None;
                let mut id: Option<String> = None;
                let mut join: Option<crate::datadogV2::model::ListConnectionsResponseDataAttributesConnectionsItemsJoin> = None;
                let mut metadata: Option<std::collections::BTreeMap<String, String>> = None;
                let mut type_: Option<String> = None;
                let mut updated_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut updated_by: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_by" => {
                            if v.is_null() {
                                continue;
                            }
                            created_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "fields" => {
                            if v.is_null() {
                                continue;
                            }
                            fields = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "join" => {
                            if v.is_null() {
                                continue;
                            }
                            join = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metadata" => {
                            if v.is_null() {
                                continue;
                            }
                            metadata = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_at" => {
                            if v.is_null() {
                                continue;
                            }
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_by" => {
                            if v.is_null() {
                                continue;
                            }
                            updated_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ListConnectionsResponseDataAttributesConnectionsItems {
                    created_at,
                    created_by,
                    fields,
                    id,
                    join,
                    metadata,
                    type_,
                    updated_at,
                    updated_by,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ListConnectionsResponseDataAttributesConnectionsItemsVisitor)
    }
}

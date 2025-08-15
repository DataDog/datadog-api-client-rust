// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `ItemApiPayloadMetaSchema` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ItemApiPayloadMetaSchema {
    /// The `ItemApiPayloadMetaSchema` `fields`.
    #[serde(rename = "fields")]
    pub fields: Option<Vec<crate::datadogV2::model::ItemApiPayloadMetaSchemaField>>,
    /// The `ItemApiPayloadMetaSchema` `primary_key`.
    #[serde(rename = "primary_key")]
    pub primary_key: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ItemApiPayloadMetaSchema {
    pub fn new() -> ItemApiPayloadMetaSchema {
        ItemApiPayloadMetaSchema {
            fields: None,
            primary_key: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn fields(
        mut self,
        value: Vec<crate::datadogV2::model::ItemApiPayloadMetaSchemaField>,
    ) -> Self {
        self.fields = Some(value);
        self
    }

    pub fn primary_key(mut self, value: String) -> Self {
        self.primary_key = Some(value);
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

impl Default for ItemApiPayloadMetaSchema {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ItemApiPayloadMetaSchema {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ItemApiPayloadMetaSchemaVisitor;
        impl<'a> Visitor<'a> for ItemApiPayloadMetaSchemaVisitor {
            type Value = ItemApiPayloadMetaSchema;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut fields: Option<
                    Vec<crate::datadogV2::model::ItemApiPayloadMetaSchemaField>,
                > = None;
                let mut primary_key: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "fields" => {
                            if v.is_null() {
                                continue;
                            }
                            fields = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "primary_key" => {
                            if v.is_null() {
                                continue;
                            }
                            primary_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ItemApiPayloadMetaSchema {
                    fields,
                    primary_key,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ItemApiPayloadMetaSchemaVisitor)
    }
}

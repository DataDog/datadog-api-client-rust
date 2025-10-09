// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `TableResultV2DataAttributesSchema` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TableResultV2DataAttributesSchema {
    /// The `schema` `fields`.
    #[serde(rename = "fields")]
    pub fields: Vec<crate::datadogV2::model::TableResultV2DataAttributesSchemaFieldsItems>,
    /// List of field names that serve as primary keys for the table. Only one primary key is supported, and it is used as an ID to retrieve rows.
    #[serde(rename = "primary_keys")]
    pub primary_keys: Vec<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TableResultV2DataAttributesSchema {
    pub fn new(
        fields: Vec<crate::datadogV2::model::TableResultV2DataAttributesSchemaFieldsItems>,
        primary_keys: Vec<String>,
    ) -> TableResultV2DataAttributesSchema {
        TableResultV2DataAttributesSchema {
            fields,
            primary_keys,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for TableResultV2DataAttributesSchema {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TableResultV2DataAttributesSchemaVisitor;
        impl<'a> Visitor<'a> for TableResultV2DataAttributesSchemaVisitor {
            type Value = TableResultV2DataAttributesSchema;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut fields: Option<
                    Vec<crate::datadogV2::model::TableResultV2DataAttributesSchemaFieldsItems>,
                > = None;
                let mut primary_keys: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "fields" => {
                            fields = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "primary_keys" => {
                            primary_keys =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let fields = fields.ok_or_else(|| M::Error::missing_field("fields"))?;
                let primary_keys =
                    primary_keys.ok_or_else(|| M::Error::missing_field("primary_keys"))?;

                let content = TableResultV2DataAttributesSchema {
                    fields,
                    primary_keys,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TableResultV2DataAttributesSchemaVisitor)
    }
}

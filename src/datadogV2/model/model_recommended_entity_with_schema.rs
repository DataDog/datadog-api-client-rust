// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A recommended entity with its schema definition.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RecommendedEntityWithSchema {
    /// Unique identifier for the recommended entity.
    #[serde(rename = "id")]
    pub id: String,
    /// Entity schema v3.
    #[serde(rename = "schema")]
    pub schema: crate::datadogV2::model::EntityV3,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RecommendedEntityWithSchema {
    pub fn new(
        id: String,
        schema: crate::datadogV2::model::EntityV3,
    ) -> RecommendedEntityWithSchema {
        RecommendedEntityWithSchema {
            id,
            schema,
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

impl<'de> Deserialize<'de> for RecommendedEntityWithSchema {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RecommendedEntityWithSchemaVisitor;
        impl<'a> Visitor<'a> for RecommendedEntityWithSchemaVisitor {
            type Value = RecommendedEntityWithSchema;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut id: Option<String> = None;
                let mut schema: Option<crate::datadogV2::model::EntityV3> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "schema" => {
                            schema = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _schema) = schema {
                                match _schema {
                                    crate::datadogV2::model::EntityV3::UnparsedObject(_schema) => {
                                        _unparsed = true;
                                    }
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
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let schema = schema.ok_or_else(|| M::Error::missing_field("schema"))?;

                let content = RecommendedEntityWithSchema {
                    id,
                    schema,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RecommendedEntityWithSchemaVisitor)
    }
}

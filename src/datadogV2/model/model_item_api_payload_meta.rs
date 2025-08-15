// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `ItemApiPayloadMeta` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ItemApiPayloadMeta {
    /// The definition of `ItemApiPayloadMetaPage` object.
    #[serde(rename = "page")]
    pub page: Option<crate::datadogV2::model::ItemApiPayloadMetaPage>,
    /// The definition of `ItemApiPayloadMetaSchema` object.
    #[serde(rename = "schema")]
    pub schema: Option<crate::datadogV2::model::ItemApiPayloadMetaSchema>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ItemApiPayloadMeta {
    pub fn new() -> ItemApiPayloadMeta {
        ItemApiPayloadMeta {
            page: None,
            schema: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn page(mut self, value: crate::datadogV2::model::ItemApiPayloadMetaPage) -> Self {
        self.page = Some(value);
        self
    }

    pub fn schema(mut self, value: crate::datadogV2::model::ItemApiPayloadMetaSchema) -> Self {
        self.schema = Some(value);
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

impl Default for ItemApiPayloadMeta {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ItemApiPayloadMeta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ItemApiPayloadMetaVisitor;
        impl<'a> Visitor<'a> for ItemApiPayloadMetaVisitor {
            type Value = ItemApiPayloadMeta;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut page: Option<crate::datadogV2::model::ItemApiPayloadMetaPage> = None;
                let mut schema: Option<crate::datadogV2::model::ItemApiPayloadMetaSchema> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "page" => {
                            if v.is_null() {
                                continue;
                            }
                            page = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "schema" => {
                            if v.is_null() {
                                continue;
                            }
                            schema = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ItemApiPayloadMeta {
                    page,
                    schema,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ItemApiPayloadMetaVisitor)
    }
}

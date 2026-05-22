// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Metadata returned alongside the entity context response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct EntityContextResponseMeta {
    /// Pagination metadata for the entity context response.
    #[serde(rename = "page")]
    pub page: crate::datadogV2::model::EntityContextPage,
    /// The total number of entities matching the query, irrespective of pagination.
    #[serde(rename = "total_count")]
    pub total_count: i32,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl EntityContextResponseMeta {
    pub fn new(
        page: crate::datadogV2::model::EntityContextPage,
        total_count: i32,
    ) -> EntityContextResponseMeta {
        EntityContextResponseMeta {
            page,
            total_count,
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

impl<'de> Deserialize<'de> for EntityContextResponseMeta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EntityContextResponseMetaVisitor;
        impl<'a> Visitor<'a> for EntityContextResponseMetaVisitor {
            type Value = EntityContextResponseMeta;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut page: Option<crate::datadogV2::model::EntityContextPage> = None;
                let mut total_count: Option<i32> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "page" => {
                            page = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_count" => {
                            total_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let page = page.ok_or_else(|| M::Error::missing_field("page"))?;
                let total_count =
                    total_count.ok_or_else(|| M::Error::missing_field("total_count"))?;

                let content = EntityContextResponseMeta {
                    page,
                    total_count,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(EntityContextResponseMetaVisitor)
    }
}

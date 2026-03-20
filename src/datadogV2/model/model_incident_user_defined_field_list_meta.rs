// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Pagination metadata for the user-defined field list response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentUserDefinedFieldListMeta {
    /// The offset of the current page.
    #[serde(rename = "page_offset")]
    pub page_offset: i64,
    /// The total number of items in the current page.
    #[serde(rename = "total")]
    pub total: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentUserDefinedFieldListMeta {
    pub fn new(page_offset: i64, total: i64) -> IncidentUserDefinedFieldListMeta {
        IncidentUserDefinedFieldListMeta {
            page_offset,
            total,
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

impl<'de> Deserialize<'de> for IncidentUserDefinedFieldListMeta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentUserDefinedFieldListMetaVisitor;
        impl<'a> Visitor<'a> for IncidentUserDefinedFieldListMetaVisitor {
            type Value = IncidentUserDefinedFieldListMeta;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut page_offset: Option<i64> = None;
                let mut total: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "page_offset" => {
                            page_offset =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total" => {
                            total = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let page_offset =
                    page_offset.ok_or_else(|| M::Error::missing_field("page_offset"))?;
                let total = total.ok_or_else(|| M::Error::missing_field("total"))?;

                let content = IncidentUserDefinedFieldListMeta {
                    page_offset,
                    total,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentUserDefinedFieldListMetaVisitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a Cloud Cost Management tag source.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CostTagKeySourceAttributes {
    /// The tag key name.
    #[serde(rename = "tag_key")]
    pub tag_key: String,
    /// Origins where this tag key was observed (for example, `aws-user-defined`).
    #[serde(rename = "tag_sources")]
    pub tag_sources: Vec<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CostTagKeySourceAttributes {
    pub fn new(tag_key: String, tag_sources: Vec<String>) -> CostTagKeySourceAttributes {
        CostTagKeySourceAttributes {
            tag_key,
            tag_sources,
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

impl<'de> Deserialize<'de> for CostTagKeySourceAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CostTagKeySourceAttributesVisitor;
        impl<'a> Visitor<'a> for CostTagKeySourceAttributesVisitor {
            type Value = CostTagKeySourceAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut tag_key: Option<String> = None;
                let mut tag_sources: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "tag_key" => {
                            tag_key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tag_sources" => {
                            tag_sources =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let tag_key = tag_key.ok_or_else(|| M::Error::missing_field("tag_key"))?;
                let tag_sources =
                    tag_sources.ok_or_else(|| M::Error::missing_field("tag_sources"))?;

                let content = CostTagKeySourceAttributes {
                    tag_key,
                    tag_sources,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CostTagKeySourceAttributesVisitor)
    }
}

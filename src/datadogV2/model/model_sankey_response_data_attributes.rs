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
pub struct SankeyResponseDataAttributes {
    #[serde(rename = "links")]
    pub links: Option<Vec<crate::datadogV2::model::SankeyResponseDataAttributesLinksItems>>,
    #[serde(rename = "nodes")]
    pub nodes: Option<Vec<crate::datadogV2::model::SankeyResponseDataAttributesNodesItems>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SankeyResponseDataAttributes {
    pub fn new() -> SankeyResponseDataAttributes {
        SankeyResponseDataAttributes {
            links: None,
            nodes: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn links(
        mut self,
        value: Vec<crate::datadogV2::model::SankeyResponseDataAttributesLinksItems>,
    ) -> Self {
        self.links = Some(value);
        self
    }

    pub fn nodes(
        mut self,
        value: Vec<crate::datadogV2::model::SankeyResponseDataAttributesNodesItems>,
    ) -> Self {
        self.nodes = Some(value);
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

impl Default for SankeyResponseDataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SankeyResponseDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SankeyResponseDataAttributesVisitor;
        impl<'a> Visitor<'a> for SankeyResponseDataAttributesVisitor {
            type Value = SankeyResponseDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut links: Option<
                    Vec<crate::datadogV2::model::SankeyResponseDataAttributesLinksItems>,
                > = None;
                let mut nodes: Option<
                    Vec<crate::datadogV2::model::SankeyResponseDataAttributesNodesItems>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "links" => {
                            if v.is_null() {
                                continue;
                            }
                            links = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "nodes" => {
                            if v.is_null() {
                                continue;
                            }
                            nodes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SankeyResponseDataAttributes {
                    links,
                    nodes,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SankeyResponseDataAttributesVisitor)
    }
}

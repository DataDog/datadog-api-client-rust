// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A link between two nodes in the Sankey diagram.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductAnalyticsSankeyLink {
    /// The step column of the source node.
    #[serde(rename = "column")]
    pub column: Option<i64>,
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The source node ID.
    #[serde(rename = "source")]
    pub source: Option<String>,
    /// The target node ID.
    #[serde(rename = "target")]
    pub target: Option<String>,
    /// The number of sessions through this link.
    #[serde(rename = "value")]
    pub value: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductAnalyticsSankeyLink {
    pub fn new() -> ProductAnalyticsSankeyLink {
        ProductAnalyticsSankeyLink {
            column: None,
            id: None,
            source: None,
            target: None,
            value: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn column(mut self, value: i64) -> Self {
        self.column = Some(value);
        self
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }

    pub fn source(mut self, value: String) -> Self {
        self.source = Some(value);
        self
    }

    pub fn target(mut self, value: String) -> Self {
        self.target = Some(value);
        self
    }

    pub fn value(mut self, value: i64) -> Self {
        self.value = Some(value);
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

impl Default for ProductAnalyticsSankeyLink {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ProductAnalyticsSankeyLink {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsSankeyLinkVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsSankeyLinkVisitor {
            type Value = ProductAnalyticsSankeyLink;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut column: Option<i64> = None;
                let mut id: Option<String> = None;
                let mut source: Option<String> = None;
                let mut target: Option<String> = None;
                let mut value: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "column" => {
                            if v.is_null() {
                                continue;
                            }
                            column = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source" => {
                            if v.is_null() {
                                continue;
                            }
                            source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "target" => {
                            if v.is_null() {
                                continue;
                            }
                            target = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "value" => {
                            if v.is_null() {
                                continue;
                            }
                            value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ProductAnalyticsSankeyLink {
                    column,
                    id,
                    source,
                    target,
                    value,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductAnalyticsSankeyLinkVisitor)
    }
}

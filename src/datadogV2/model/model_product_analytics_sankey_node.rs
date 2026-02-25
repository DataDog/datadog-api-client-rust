// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A node in the Sankey diagram representing a page or aggregation.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductAnalyticsSankeyNode {
    /// Nodes aggregated into this node (for "other" type).
    #[serde(rename = "aggregated_nodes")]
    pub aggregated_nodes:
        Option<Vec<crate::datadogV2::model::ProductAnalyticsSankeyAggregatedNode>>,
    /// The step column (0-indexed).
    #[serde(rename = "column")]
    pub column: Option<i64>,
    #[serde(rename = "dropoff_value")]
    pub dropoff_value: Option<i64>,
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[serde(rename = "incoming_value")]
    pub incoming_value: Option<i64>,
    /// The page name.
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(rename = "outgoing_value")]
    pub outgoing_value: Option<i64>,
    /// Node type.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::ProductAnalyticsSankeyNodeType>,
    /// The number of sessions through this node.
    #[serde(rename = "value")]
    pub value: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductAnalyticsSankeyNode {
    pub fn new() -> ProductAnalyticsSankeyNode {
        ProductAnalyticsSankeyNode {
            aggregated_nodes: None,
            column: None,
            dropoff_value: None,
            id: None,
            incoming_value: None,
            name: None,
            outgoing_value: None,
            type_: None,
            value: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn aggregated_nodes(
        mut self,
        value: Vec<crate::datadogV2::model::ProductAnalyticsSankeyAggregatedNode>,
    ) -> Self {
        self.aggregated_nodes = Some(value);
        self
    }

    pub fn column(mut self, value: i64) -> Self {
        self.column = Some(value);
        self
    }

    pub fn dropoff_value(mut self, value: i64) -> Self {
        self.dropoff_value = Some(value);
        self
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }

    pub fn incoming_value(mut self, value: i64) -> Self {
        self.incoming_value = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn outgoing_value(mut self, value: i64) -> Self {
        self.outgoing_value = Some(value);
        self
    }

    pub fn type_(mut self, value: crate::datadogV2::model::ProductAnalyticsSankeyNodeType) -> Self {
        self.type_ = Some(value);
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

impl Default for ProductAnalyticsSankeyNode {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ProductAnalyticsSankeyNode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsSankeyNodeVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsSankeyNodeVisitor {
            type Value = ProductAnalyticsSankeyNode;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut aggregated_nodes: Option<
                    Vec<crate::datadogV2::model::ProductAnalyticsSankeyAggregatedNode>,
                > = None;
                let mut column: Option<i64> = None;
                let mut dropoff_value: Option<i64> = None;
                let mut id: Option<String> = None;
                let mut incoming_value: Option<i64> = None;
                let mut name: Option<String> = None;
                let mut outgoing_value: Option<i64> = None;
                let mut type_: Option<crate::datadogV2::model::ProductAnalyticsSankeyNodeType> =
                    None;
                let mut value: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "aggregated_nodes" => {
                            if v.is_null() {
                                continue;
                            }
                            aggregated_nodes =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "column" => {
                            if v.is_null() {
                                continue;
                            }
                            column = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "dropoff_value" => {
                            if v.is_null() {
                                continue;
                            }
                            dropoff_value =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "incoming_value" => {
                            if v.is_null() {
                                continue;
                            }
                            incoming_value =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "outgoing_value" => {
                            if v.is_null() {
                                continue;
                            }
                            outgoing_value =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::ProductAnalyticsSankeyNodeType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
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

                let content = ProductAnalyticsSankeyNode {
                    aggregated_nodes,
                    column,
                    dropoff_value,
                    id,
                    incoming_value,
                    name,
                    outgoing_value,
                    type_,
                    value,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductAnalyticsSankeyNodeVisitor)
    }
}

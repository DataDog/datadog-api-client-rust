// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A timeseries object containing `queries` and `formulas` arrays.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UnitCostQueryDefinition {
    /// The list of formulas applied to the queries for this side of the ratio.
    #[serde(rename = "formulas")]
    pub formulas: Vec<std::collections::BTreeMap<String, serde_json::Value>>,
    /// The list of queries evaluated for this side of the ratio.
    #[serde(rename = "queries")]
    pub queries: Vec<std::collections::BTreeMap<String, serde_json::Value>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UnitCostQueryDefinition {
    pub fn new(
        formulas: Vec<std::collections::BTreeMap<String, serde_json::Value>>,
        queries: Vec<std::collections::BTreeMap<String, serde_json::Value>>,
    ) -> UnitCostQueryDefinition {
        UnitCostQueryDefinition {
            formulas,
            queries,
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

impl<'de> Deserialize<'de> for UnitCostQueryDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UnitCostQueryDefinitionVisitor;
        impl<'a> Visitor<'a> for UnitCostQueryDefinitionVisitor {
            type Value = UnitCostQueryDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut formulas: Option<
                    Vec<std::collections::BTreeMap<String, serde_json::Value>>,
                > = None;
                let mut queries: Option<
                    Vec<std::collections::BTreeMap<String, serde_json::Value>>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "formulas" => {
                            formulas = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "queries" => {
                            queries = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let formulas = formulas.ok_or_else(|| M::Error::missing_field("formulas"))?;
                let queries = queries.ok_or_else(|| M::Error::missing_field("queries"))?;

                let content = UnitCostQueryDefinition {
                    formulas,
                    queries,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UnitCostQueryDefinitionVisitor)
    }
}

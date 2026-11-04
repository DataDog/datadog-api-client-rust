// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The query that produces the response values, in the same format as dashboard and notebook widget
/// requests, so a client can graph the values in a widget. Each formula's `id` names the response value
/// that it produces.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CommitmentsQueryDefinition {
    /// The formulas of a query definition.
    #[serde(rename = "formulas")]
    pub formulas: Vec<crate::datadogV2::model::CommitmentsQueryDefinitionFormula>,
    /// The metric queries of a query definition.
    #[serde(rename = "queries")]
    pub queries: Vec<crate::datadogV2::model::CommitmentsQueryDefinitionQuery>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CommitmentsQueryDefinition {
    pub fn new(
        formulas: Vec<crate::datadogV2::model::CommitmentsQueryDefinitionFormula>,
        queries: Vec<crate::datadogV2::model::CommitmentsQueryDefinitionQuery>,
    ) -> CommitmentsQueryDefinition {
        CommitmentsQueryDefinition {
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

impl<'de> Deserialize<'de> for CommitmentsQueryDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CommitmentsQueryDefinitionVisitor;
        impl<'a> Visitor<'a> for CommitmentsQueryDefinitionVisitor {
            type Value = CommitmentsQueryDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut formulas: Option<
                    Vec<crate::datadogV2::model::CommitmentsQueryDefinitionFormula>,
                > = None;
                let mut queries: Option<
                    Vec<crate::datadogV2::model::CommitmentsQueryDefinitionQuery>,
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

                let content = CommitmentsQueryDefinition {
                    formulas,
                    queries,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CommitmentsQueryDefinitionVisitor)
    }
}

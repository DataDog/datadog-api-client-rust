// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The queries and formula used to calculate the SLI value.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SLOTimeSliceQuery {
    /// A list that contains exactly one formula, as only a single formula may be used in a time-slice SLO.
    #[serde(rename = "formulas")]
    pub formulas: Vec<crate::datadogV1::model::SLOFormula>,
    /// A list of queries that are used to calculate the SLI value.
    #[serde(rename = "queries")]
    pub queries: Vec<crate::datadogV1::model::SLODataSourceQueryDefinition>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SLOTimeSliceQuery {
    pub fn new(
        formulas: Vec<crate::datadogV1::model::SLOFormula>,
        queries: Vec<crate::datadogV1::model::SLODataSourceQueryDefinition>,
    ) -> SLOTimeSliceQuery {
        SLOTimeSliceQuery {
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

impl<'de> Deserialize<'de> for SLOTimeSliceQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SLOTimeSliceQueryVisitor;
        impl<'a> Visitor<'a> for SLOTimeSliceQueryVisitor {
            type Value = SLOTimeSliceQuery;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut formulas: Option<Vec<crate::datadogV1::model::SLOFormula>> = None;
                let mut queries: Option<
                    Vec<crate::datadogV1::model::SLODataSourceQueryDefinition>,
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

                let content = SLOTimeSliceQuery {
                    formulas,
                    queries,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SLOTimeSliceQueryVisitor)
    }
}

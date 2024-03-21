// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A formula for calculation based on one or more queries.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct QueryFormula {
    /// Formula string, referencing one or more queries with their name property.
    #[serde(rename = "formula")]
    pub formula: String,
    /// Message for specifying limits to the number of values returned by a query.
    /// This limit is only for scalar queries and has no effect on timeseries queries.
    #[serde(rename = "limit")]
    pub limit: Option<crate::datadogV2::model::FormulaLimit>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl QueryFormula {
    pub fn new(formula: String) -> QueryFormula {
        QueryFormula {
            formula,
            limit: None,
            _unparsed: false,
        }
    }

    pub fn limit(mut self, value: crate::datadogV2::model::FormulaLimit) -> Self {
        self.limit = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for QueryFormula {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct QueryFormulaVisitor;
        impl<'a> Visitor<'a> for QueryFormulaVisitor {
            type Value = QueryFormula;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut formula: Option<String> = None;
                let mut limit: Option<crate::datadogV2::model::FormulaLimit> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "formula" => {
                            formula = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "limit" => {
                            if v.is_null() {
                                continue;
                            }
                            limit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let formula = formula.ok_or_else(|| M::Error::missing_field("formula"))?;

                let content = QueryFormula {
                    formula,
                    limit,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(QueryFormulaVisitor)
    }
}

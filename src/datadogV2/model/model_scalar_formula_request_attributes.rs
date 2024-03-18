// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The object describing a scalar formula request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ScalarFormulaRequestAttributes {
    /// List of formulas to be calculated and returned as responses.
    #[serde(rename = "formulas")]
    pub formulas: Option<Vec<crate::datadogV2::model::QueryFormula>>,
    /// Start date (inclusive) of the query in milliseconds since the Unix epoch.
    #[serde(rename = "from")]
    pub from: i64,
    /// List of queries to be run and used as inputs to the formulas.
    #[serde(rename = "queries")]
    pub queries: Vec<crate::datadogV2::model::ScalarQuery>,
    /// End date (exclusive) of the query in milliseconds since the Unix epoch.
    #[serde(rename = "to")]
    pub to: i64,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ScalarFormulaRequestAttributes {
    pub fn new(
        from: i64,
        queries: Vec<crate::datadogV2::model::ScalarQuery>,
        to: i64,
    ) -> ScalarFormulaRequestAttributes {
        ScalarFormulaRequestAttributes {
            formulas: None,
            from,
            queries,
            to,
            _unparsed: false,
        }
    }

    pub fn formulas(mut self, value: Vec<crate::datadogV2::model::QueryFormula>) -> Self {
        self.formulas = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for ScalarFormulaRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ScalarFormulaRequestAttributesVisitor;
        impl<'a> Visitor<'a> for ScalarFormulaRequestAttributesVisitor {
            type Value = ScalarFormulaRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut formulas: Option<Vec<crate::datadogV2::model::QueryFormula>> = None;
                let mut from: Option<i64> = None;
                let mut queries: Option<Vec<crate::datadogV2::model::ScalarQuery>> = None;
                let mut to: Option<i64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "formulas" => {
                            if v.is_null() {
                                continue;
                            }
                            formulas = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "from" => {
                            from = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "queries" => {
                            queries = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "to" => {
                            to = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let from = from.ok_or_else(|| M::Error::missing_field("from"))?;
                let queries = queries.ok_or_else(|| M::Error::missing_field("queries"))?;
                let to = to.ok_or_else(|| M::Error::missing_field("to"))?;

                let content = ScalarFormulaRequestAttributes {
                    formulas,
                    from,
                    queries,
                    to,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ScalarFormulaRequestAttributesVisitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// List of objects used to group by.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FormulaAndFunctionEventQueryGroupBy {
    /// Event facet.
    #[serde(rename = "facet")]
    pub facet: String,
    /// Number of groups to return.
    #[serde(rename = "limit")]
    pub limit: Option<i64>,
    /// Options for sorting group by results.
    #[serde(rename = "sort")]
    pub sort: Option<crate::datadogV1::model::FormulaAndFunctionEventQueryGroupBySort>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FormulaAndFunctionEventQueryGroupBy {
    pub fn new(facet: String) -> FormulaAndFunctionEventQueryGroupBy {
        FormulaAndFunctionEventQueryGroupBy {
            facet,
            limit: None,
            sort: None,
            _unparsed: false,
        }
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn sort(
        mut self,
        value: crate::datadogV1::model::FormulaAndFunctionEventQueryGroupBySort,
    ) -> Self {
        self.sort = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for FormulaAndFunctionEventQueryGroupBy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FormulaAndFunctionEventQueryGroupByVisitor;
        impl<'a> Visitor<'a> for FormulaAndFunctionEventQueryGroupByVisitor {
            type Value = FormulaAndFunctionEventQueryGroupBy;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut facet: Option<String> = None;
                let mut limit: Option<i64> = None;
                let mut sort: Option<
                    crate::datadogV1::model::FormulaAndFunctionEventQueryGroupBySort,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "facet" => {
                            facet = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "limit" => {
                            if v.is_null() {
                                continue;
                            }
                            limit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sort" => {
                            if v.is_null() {
                                continue;
                            }
                            sort = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let facet = facet.ok_or_else(|| M::Error::missing_field("facet"))?;

                let content = FormulaAndFunctionEventQueryGroupBy {
                    facet,
                    limit,
                    sort,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FormulaAndFunctionEventQueryGroupByVisitor)
    }
}

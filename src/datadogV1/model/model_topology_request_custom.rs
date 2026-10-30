// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Scalar request whose result rows become nodes.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TopologyRequestCustom {
    /// List of formulas that operate on queries.
    #[serde(rename = "formulas")]
    pub formulas: Option<Vec<crate::datadogV1::model::WidgetFormula>>,
    /// Mapping from result columns to node fields. Each result row feeds one node.
    #[serde(rename = "projection")]
    pub projection: crate::datadogV1::model::TopologyRequestCustomProjection,
    /// List of queries that can be returned directly or used in formulas.
    #[serde(rename = "queries")]
    pub queries: Vec<crate::datadogV1::model::FormulaAndFunctionQueryDefinition>,
    /// Response format of a custom topology map request.
    #[serde(rename = "response_format")]
    pub response_format: crate::datadogV1::model::TopologyRequestCustomResponseFormat,
    /// The controls for sorting the widget.
    #[serde(rename = "sort")]
    pub sort: Option<crate::datadogV1::model::WidgetSortBy>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TopologyRequestCustom {
    pub fn new(
        projection: crate::datadogV1::model::TopologyRequestCustomProjection,
        queries: Vec<crate::datadogV1::model::FormulaAndFunctionQueryDefinition>,
        response_format: crate::datadogV1::model::TopologyRequestCustomResponseFormat,
    ) -> TopologyRequestCustom {
        TopologyRequestCustom {
            formulas: None,
            projection,
            queries,
            response_format,
            sort: None,
            _unparsed: false,
        }
    }

    pub fn formulas(mut self, value: Vec<crate::datadogV1::model::WidgetFormula>) -> Self {
        self.formulas = Some(value);
        self
    }

    pub fn sort(mut self, value: crate::datadogV1::model::WidgetSortBy) -> Self {
        self.sort = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for TopologyRequestCustom {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TopologyRequestCustomVisitor;
        impl<'a> Visitor<'a> for TopologyRequestCustomVisitor {
            type Value = TopologyRequestCustom;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut formulas: Option<Vec<crate::datadogV1::model::WidgetFormula>> = None;
                let mut projection: Option<
                    crate::datadogV1::model::TopologyRequestCustomProjection,
                > = None;
                let mut queries: Option<
                    Vec<crate::datadogV1::model::FormulaAndFunctionQueryDefinition>,
                > = None;
                let mut response_format: Option<
                    crate::datadogV1::model::TopologyRequestCustomResponseFormat,
                > = None;
                let mut sort: Option<crate::datadogV1::model::WidgetSortBy> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "formulas" => {
                            if v.is_null() {
                                continue;
                            }
                            formulas = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "projection" => {
                            projection = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "queries" => {
                            queries = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "response_format" => {
                            response_format =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _response_format) = response_format {
                                match _response_format {
                                    crate::datadogV1::model::TopologyRequestCustomResponseFormat::UnparsedObject(_response_format) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "sort" => {
                            if v.is_null() {
                                continue;
                            }
                            sort = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let projection = projection.ok_or_else(|| M::Error::missing_field("projection"))?;
                let queries = queries.ok_or_else(|| M::Error::missing_field("queries"))?;
                let response_format =
                    response_format.ok_or_else(|| M::Error::missing_field("response_format"))?;

                let content = TopologyRequestCustom {
                    formulas,
                    projection,
                    queries,
                    response_format,
                    sort,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TopologyRequestCustomVisitor)
    }
}

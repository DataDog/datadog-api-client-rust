// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Scatterplot table request. Supports two modes:
/// - **Formulas and functions** (default): `request_type` is absent or `"table"`. Uses `queries` and `formulas`.
/// - **Data projection**: `request_type` is `"data_projection"`. Uses `query`, `projection`, and optionally `limit`.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ScatterplotTableRequest {
    /// List of Scatterplot formulas that operate on queries.
    #[serde(rename = "formulas")]
    pub formulas: Option<Vec<crate::datadogV1::model::ScatterplotWidgetFormula>>,
    /// Maximum number of rows to return. Used when `request_type` is `"data_projection"`.
    #[serde(rename = "limit")]
    pub limit: Option<i64>,
    /// The projection configuration for a scatterplot data projection request.
    #[serde(rename = "projection")]
    pub projection: Option<crate::datadogV1::model::ScatterplotDataProjectionProjection>,
    /// List of queries that can be returned directly or used in formulas.
    #[serde(rename = "queries")]
    pub queries: Option<Vec<crate::datadogV1::model::FormulaAndFunctionQueryDefinition>>,
    /// The query for a scatterplot data projection request.
    #[serde(rename = "query")]
    pub query: Option<crate::datadogV1::model::ScatterplotDataProjectionQuery>,
    /// The type of the scatterplot table request.
    #[serde(rename = "request_type")]
    pub request_type: Option<crate::datadogV1::model::ScatterplotTableRequestType>,
    /// Timeseries, scalar, or event list response. Event list response formats are supported by Geomap widgets.
    #[serde(rename = "response_format")]
    pub response_format: Option<crate::datadogV1::model::FormulaAndFunctionResponseFormat>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ScatterplotTableRequest {
    pub fn new() -> ScatterplotTableRequest {
        ScatterplotTableRequest {
            formulas: None,
            limit: None,
            projection: None,
            queries: None,
            query: None,
            request_type: None,
            response_format: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn formulas(
        mut self,
        value: Vec<crate::datadogV1::model::ScatterplotWidgetFormula>,
    ) -> Self {
        self.formulas = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn projection(
        mut self,
        value: crate::datadogV1::model::ScatterplotDataProjectionProjection,
    ) -> Self {
        self.projection = Some(value);
        self
    }

    pub fn queries(
        mut self,
        value: Vec<crate::datadogV1::model::FormulaAndFunctionQueryDefinition>,
    ) -> Self {
        self.queries = Some(value);
        self
    }

    pub fn query(mut self, value: crate::datadogV1::model::ScatterplotDataProjectionQuery) -> Self {
        self.query = Some(value);
        self
    }

    pub fn request_type(
        mut self,
        value: crate::datadogV1::model::ScatterplotTableRequestType,
    ) -> Self {
        self.request_type = Some(value);
        self
    }

    pub fn response_format(
        mut self,
        value: crate::datadogV1::model::FormulaAndFunctionResponseFormat,
    ) -> Self {
        self.response_format = Some(value);
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

impl Default for ScatterplotTableRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ScatterplotTableRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ScatterplotTableRequestVisitor;
        impl<'a> Visitor<'a> for ScatterplotTableRequestVisitor {
            type Value = ScatterplotTableRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut formulas: Option<Vec<crate::datadogV1::model::ScatterplotWidgetFormula>> =
                    None;
                let mut limit: Option<i64> = None;
                let mut projection: Option<
                    crate::datadogV1::model::ScatterplotDataProjectionProjection,
                > = None;
                let mut queries: Option<
                    Vec<crate::datadogV1::model::FormulaAndFunctionQueryDefinition>,
                > = None;
                let mut query: Option<crate::datadogV1::model::ScatterplotDataProjectionQuery> =
                    None;
                let mut request_type: Option<crate::datadogV1::model::ScatterplotTableRequestType> =
                    None;
                let mut response_format: Option<
                    crate::datadogV1::model::FormulaAndFunctionResponseFormat,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "formulas" => {
                            if v.is_null() {
                                continue;
                            }
                            formulas = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "limit" => {
                            if v.is_null() {
                                continue;
                            }
                            limit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "projection" => {
                            if v.is_null() {
                                continue;
                            }
                            projection = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "queries" => {
                            if v.is_null() {
                                continue;
                            }
                            queries = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query" => {
                            if v.is_null() {
                                continue;
                            }
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "request_type" => {
                            if v.is_null() {
                                continue;
                            }
                            request_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _request_type) = request_type {
                                match _request_type {
                                    crate::datadogV1::model::ScatterplotTableRequestType::UnparsedObject(_request_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "response_format" => {
                            if v.is_null() {
                                continue;
                            }
                            response_format =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _response_format) = response_format {
                                match _response_format {
                                    crate::datadogV1::model::FormulaAndFunctionResponseFormat::UnparsedObject(_response_format) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ScatterplotTableRequest {
                    formulas,
                    limit,
                    projection,
                    queries,
                    query,
                    request_type,
                    response_format,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ScatterplotTableRequestVisitor)
    }
}

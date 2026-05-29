// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Scalar formula request for the infrastructure host map widget. Each formula specifies
/// which visual dimension it drives.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct HostMapWidgetScalarRequest {
    /// List of formulas that operate on queries, each assigned to a visual dimension.
    #[serde(rename = "formulas")]
    pub formulas: Vec<crate::datadogV1::model::HostMapWidgetFormula>,
    /// List of queries that can be returned directly or used in formulas.
    #[serde(rename = "queries")]
    pub queries: Vec<crate::datadogV1::model::FormulaAndFunctionQueryDefinition>,
    /// Response format for the scalar formula request. Only `scalar` is supported.
    #[serde(rename = "response_format")]
    pub response_format: crate::datadogV1::model::HostMapWidgetScalarRequestResponseFormat,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl HostMapWidgetScalarRequest {
    pub fn new(
        formulas: Vec<crate::datadogV1::model::HostMapWidgetFormula>,
        queries: Vec<crate::datadogV1::model::FormulaAndFunctionQueryDefinition>,
        response_format: crate::datadogV1::model::HostMapWidgetScalarRequestResponseFormat,
    ) -> HostMapWidgetScalarRequest {
        HostMapWidgetScalarRequest {
            formulas,
            queries,
            response_format,
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

impl<'de> Deserialize<'de> for HostMapWidgetScalarRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HostMapWidgetScalarRequestVisitor;
        impl<'a> Visitor<'a> for HostMapWidgetScalarRequestVisitor {
            type Value = HostMapWidgetScalarRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut formulas: Option<Vec<crate::datadogV1::model::HostMapWidgetFormula>> = None;
                let mut queries: Option<
                    Vec<crate::datadogV1::model::FormulaAndFunctionQueryDefinition>,
                > = None;
                let mut response_format: Option<
                    crate::datadogV1::model::HostMapWidgetScalarRequestResponseFormat,
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
                        "response_format" => {
                            response_format =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _response_format) = response_format {
                                match _response_format {
                                    crate::datadogV1::model::HostMapWidgetScalarRequestResponseFormat::UnparsedObject(_response_format) => {
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
                let formulas = formulas.ok_or_else(|| M::Error::missing_field("formulas"))?;
                let queries = queries.ok_or_else(|| M::Error::missing_field("queries"))?;
                let response_format =
                    response_format.ok_or_else(|| M::Error::missing_field("response_format"))?;

                let content = HostMapWidgetScalarRequest {
                    formulas,
                    queries,
                    response_format,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(HostMapWidgetScalarRequestVisitor)
    }
}

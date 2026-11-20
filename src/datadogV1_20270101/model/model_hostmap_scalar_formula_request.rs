// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct HostmapScalarFormulaRequest {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "formulas")]
    pub formulas: Vec<crate::datadogV1_20270101::model::HostmapFormula>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "queries")]
    pub queries: Vec<crate::datadogV1_20270101::model::FormulaAndFunctionQueryDefinition>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "response_format")]
    pub response_format: crate::datadogV1_20270101::model::HostmapScalarFormulaRequestResponseFormat,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "sort")]
    pub sort: Option<crate::datadogV1_20270101::model::ScalarFormulaSort>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl HostmapScalarFormulaRequest {
    pub fn new(formulas: Vec<crate::datadogV1_20270101::model::HostmapFormula>, queries: Vec<crate::datadogV1_20270101::model::FormulaAndFunctionQueryDefinition>, response_format: crate::datadogV1_20270101::model::HostmapScalarFormulaRequestResponseFormat) -> HostmapScalarFormulaRequest {
        HostmapScalarFormulaRequest {
            formulas,
            queries,
            response_format,
            sort: None,
            _unparsed: false,
        }
    }
    
    pub fn sort(mut self, value: crate::datadogV1_20270101::model::ScalarFormulaSort) -> Self {
        self.sort = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for HostmapScalarFormulaRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HostmapScalarFormulaRequestVisitor;
        impl<'a> Visitor<'a> for HostmapScalarFormulaRequestVisitor {
            type Value = HostmapScalarFormulaRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut formulas: Option<Vec<crate::datadogV1_20270101::model::HostmapFormula>> = None;
                let mut queries: Option<Vec<crate::datadogV1_20270101::model::FormulaAndFunctionQueryDefinition>> = None;
                let mut response_format: Option<crate::datadogV1_20270101::model::HostmapScalarFormulaRequestResponseFormat> = None;
                let mut sort: Option<crate::datadogV1_20270101::model::ScalarFormulaSort> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "formulas" => {
                            formulas = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "queries" => {
                            queries = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "response_format" => {
                            response_format = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _response_format) = response_format {
                                match _response_format {
                                    crate::datadogV1_20270101::model::HostmapScalarFormulaRequestResponseFormat::UnparsedObject(_response_format) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "sort" => {
                            if v.is_null() {
                                continue;
                            }
                            sort = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let formulas = formulas.ok_or_else(|| M::Error::missing_field("formulas"))?;
                let queries = queries.ok_or_else(|| M::Error::missing_field("queries"))?;
                let response_format = response_format.ok_or_else(|| M::Error::missing_field("response_format"))?;

                let content = HostmapScalarFormulaRequest {
                    formulas,
                    queries,
                    response_format,
                    sort,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(HostmapScalarFormulaRequestVisitor)
    }
}

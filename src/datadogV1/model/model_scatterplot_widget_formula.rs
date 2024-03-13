// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Formula to be used in a Scatterplot widget query.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ScatterplotWidgetFormula {
    /// Expression alias.
    #[serde(rename = "alias")]
    pub alias: Option<String>,
    /// Dimension of the Scatterplot.
    #[serde(rename = "dimension")]
    pub dimension: crate::datadogV1::model::ScatterplotDimension,
    /// String expression built from queries, formulas, and functions.
    #[serde(rename = "formula")]
    pub formula: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ScatterplotWidgetFormula {
    pub fn new(
        dimension: crate::datadogV1::model::ScatterplotDimension,
        formula: String,
    ) -> ScatterplotWidgetFormula {
        ScatterplotWidgetFormula {
            alias: None,
            dimension,
            formula,
            _unparsed: false,
        }
    }

    pub fn alias(mut self, value: String) -> Self {
        self.alias = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for ScatterplotWidgetFormula {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ScatterplotWidgetFormulaVisitor;
        impl<'a> Visitor<'a> for ScatterplotWidgetFormulaVisitor {
            type Value = ScatterplotWidgetFormula;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut alias: Option<String> = None;
                let mut dimension: Option<crate::datadogV1::model::ScatterplotDimension> = None;
                let mut formula: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "alias" => {
                            if v.is_null() {
                                continue;
                            }
                            alias = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "dimension" => {
                            dimension = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _dimension) = dimension {
                                match _dimension {
                                    crate::datadogV1::model::ScatterplotDimension::UnparsedObject(_dimension) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "formula" => {
                            formula = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let dimension = dimension.ok_or_else(|| M::Error::missing_field("dimension"))?;
                let formula = formula.ok_or_else(|| M::Error::missing_field("formula"))?;

                let content = ScatterplotWidgetFormula {
                    alias,
                    dimension,
                    formula,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ScatterplotWidgetFormulaVisitor)
    }
}

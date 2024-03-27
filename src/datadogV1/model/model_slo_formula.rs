// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A formula that specifies how to combine the results of multiple queries.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SLOFormula {
    /// The formula string, which is an expression involving named queries.
    #[serde(rename = "formula")]
    pub formula: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SLOFormula {
    pub fn new(formula: String) -> SLOFormula {
        SLOFormula {
            formula,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for SLOFormula {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SLOFormulaVisitor;
        impl<'a> Visitor<'a> for SLOFormulaVisitor {
            type Value = SLOFormula;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut formula: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "formula" => {
                            formula = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let formula = formula.ok_or_else(|| M::Error::missing_field("formula"))?;

                let content = SLOFormula { formula, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SLOFormulaVisitor)
    }
}

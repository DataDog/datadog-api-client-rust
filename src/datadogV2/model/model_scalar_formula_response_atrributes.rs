// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The object describing a scalar response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ScalarFormulaResponseAtrributes {
    /// List of response columns, each corresponding to an individual formula or query in the request and with values in parallel arrays matching the series list.
    #[serde(rename = "columns")]
    pub columns: Option<Vec<crate::datadogV2::model::ScalarColumn>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ScalarFormulaResponseAtrributes {
    pub fn new() -> ScalarFormulaResponseAtrributes {
        ScalarFormulaResponseAtrributes {
            columns: None,
            _unparsed: false,
        }
    }

    pub fn columns(mut self, value: Vec<crate::datadogV2::model::ScalarColumn>) -> Self {
        self.columns = Some(value);
        self
    }
}

impl Default for ScalarFormulaResponseAtrributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ScalarFormulaResponseAtrributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ScalarFormulaResponseAtrributesVisitor;
        impl<'a> Visitor<'a> for ScalarFormulaResponseAtrributesVisitor {
            type Value = ScalarFormulaResponseAtrributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut columns: Option<Vec<crate::datadogV2::model::ScalarColumn>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "columns" => {
                            if v.is_null() {
                                continue;
                            }
                            columns = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = ScalarFormulaResponseAtrributes { columns, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ScalarFormulaResponseAtrributesVisitor)
    }
}

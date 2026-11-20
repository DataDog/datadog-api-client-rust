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
pub struct ScalarFormulaSort {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "count")]
    pub count: Option<f64>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "offset")]
    pub offset: Option<f64>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "order_by")]
    pub order_by: Option<Vec<crate::datadogV1_20270101::model::ScalarFormulaOrderBy>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ScalarFormulaSort {
    pub fn new() -> ScalarFormulaSort {
        ScalarFormulaSort {
            count: None,
            offset: None,
            order_by: None,
            _unparsed: false,
        }
    }
    
    pub fn count(mut self, value: f64) -> Self {
        self.count = Some(value);
        self
    }
    
    pub fn offset(mut self, value: f64) -> Self {
        self.offset = Some(value);
        self
    }
    
    pub fn order_by(mut self, value: Vec<crate::datadogV1_20270101::model::ScalarFormulaOrderBy>) -> Self {
        self.order_by = Some(value);
        self
    }
    
}


impl Default for ScalarFormulaSort {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ScalarFormulaSort {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ScalarFormulaSortVisitor;
        impl<'a> Visitor<'a> for ScalarFormulaSortVisitor {
            type Value = ScalarFormulaSort;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut count: Option<f64> = None;
                let mut offset: Option<f64> = None;
                let mut order_by: Option<Vec<crate::datadogV1_20270101::model::ScalarFormulaOrderBy>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "count" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            count = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "offset" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            offset = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "order_by" => {
                            if v.is_null() {
                                continue;
                            }
                            order_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }

                let content = ScalarFormulaSort {
                    count,
                    offset,
                    order_by,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ScalarFormulaSortVisitor)
    }
}

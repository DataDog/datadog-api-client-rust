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
pub struct TableFormulaAnyOfAnyOfItemLimit4452334816 {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "count")]
    pub count: Option<f64>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "order")]
    pub order: Option<crate::datadogV1_20270101::model::QuerySortOrder>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl TableFormulaAnyOfAnyOfItemLimit4452334816 {
    pub fn new() -> TableFormulaAnyOfAnyOfItemLimit4452334816 {
        TableFormulaAnyOfAnyOfItemLimit4452334816 {
            count: None,
            order: None,
            _unparsed: false,
        }
    }
    
    pub fn count(mut self, value: f64) -> Self {
        self.count = Some(value);
        self
    }
    
    pub fn order(mut self, value: crate::datadogV1_20270101::model::QuerySortOrder) -> Self {
        self.order = Some(value);
        self
    }
    
}


impl Default for TableFormulaAnyOfAnyOfItemLimit4452334816 {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TableFormulaAnyOfAnyOfItemLimit4452334816 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TableFormulaAnyOfAnyOfItemLimit4452334816Visitor;
        impl<'a> Visitor<'a> for TableFormulaAnyOfAnyOfItemLimit4452334816Visitor {
            type Value = TableFormulaAnyOfAnyOfItemLimit4452334816;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut count: Option<f64> = None;
                let mut order: Option<crate::datadogV1_20270101::model::QuerySortOrder> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "count" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            count = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "order" => {
                            if v.is_null() {
                                continue;
                            }
                            order = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _order) = order {
                                match _order {
                                    crate::datadogV1_20270101::model::QuerySortOrder::UnparsedObject(_order) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }

                let content = TableFormulaAnyOfAnyOfItemLimit4452334816 {
                    count,
                    order,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TableFormulaAnyOfAnyOfItemLimit4452334816Visitor)
    }
}

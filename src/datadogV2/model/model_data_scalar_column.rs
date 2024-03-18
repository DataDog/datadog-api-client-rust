// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A column containing the numerical results for a formula or query.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DataScalarColumn {
    /// Metadata for the resulting numerical values.
    #[serde(rename = "meta")]
    pub meta: Option<crate::datadogV2::model::ScalarMeta>,
    /// The name referencing the formula or query for this column.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The type of column present for numbers.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::ScalarColumnTypeNumber>,
    /// The array of numerical values for one formula or query.
    #[serde(rename = "values")]
    pub values: Option<Vec<Option<f64>>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DataScalarColumn {
    pub fn new() -> DataScalarColumn {
        DataScalarColumn {
            meta: None,
            name: None,
            type_: None,
            values: None,
            _unparsed: false,
        }
    }

    pub fn meta(mut self, value: crate::datadogV2::model::ScalarMeta) -> Self {
        self.meta = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn type_(mut self, value: crate::datadogV2::model::ScalarColumnTypeNumber) -> Self {
        self.type_ = Some(value);
        self
    }

    pub fn values(mut self, value: Vec<Option<f64>>) -> Self {
        self.values = Some(value);
        self
    }
}

impl Default for DataScalarColumn {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for DataScalarColumn {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DataScalarColumnVisitor;
        impl<'a> Visitor<'a> for DataScalarColumnVisitor {
            type Value = DataScalarColumn;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut meta: Option<crate::datadogV2::model::ScalarMeta> = None;
                let mut name: Option<String> = None;
                let mut type_: Option<crate::datadogV2::model::ScalarColumnTypeNumber> = None;
                let mut values: Option<Vec<Option<f64>>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "meta" => {
                            if v.is_null() {
                                continue;
                            }
                            meta = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::ScalarColumnTypeNumber::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "values" => {
                            if v.is_null() {
                                continue;
                            }
                            values = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = DataScalarColumn {
                    meta,
                    name,
                    type_,
                    values,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DataScalarColumnVisitor)
    }
}

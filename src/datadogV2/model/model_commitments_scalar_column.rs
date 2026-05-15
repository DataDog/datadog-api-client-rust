// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A column in a scalar response. When type is "group", values contains arrays of strings. When type is "number", values contains numeric values.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CommitmentsScalarColumn {
    /// Metadata for a scalar column, including unit information.
    #[serde(rename = "meta")]
    pub meta: Option<crate::datadogV2::model::CommitmentsScalarColumnMeta>,
    /// The column name.
    #[serde(rename = "name")]
    pub name: String,
    /// The column type. "group" for dimension columns, "number" for metric columns.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::CommitmentsScalarColumnType,
    /// Values for a scalar column. Arrays of strings for group columns, numbers for value columns.
    #[serde(rename = "values")]
    pub values: Vec<serde_json::Value>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CommitmentsScalarColumn {
    pub fn new(
        name: String,
        type_: crate::datadogV2::model::CommitmentsScalarColumnType,
        values: Vec<serde_json::Value>,
    ) -> CommitmentsScalarColumn {
        CommitmentsScalarColumn {
            meta: None,
            name,
            type_,
            values,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn meta(mut self, value: crate::datadogV2::model::CommitmentsScalarColumnMeta) -> Self {
        self.meta = Some(value);
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

impl<'de> Deserialize<'de> for CommitmentsScalarColumn {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CommitmentsScalarColumnVisitor;
        impl<'a> Visitor<'a> for CommitmentsScalarColumnVisitor {
            type Value = CommitmentsScalarColumn;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut meta: Option<crate::datadogV2::model::CommitmentsScalarColumnMeta> = None;
                let mut name: Option<String> = None;
                let mut type_: Option<crate::datadogV2::model::CommitmentsScalarColumnType> = None;
                let mut values: Option<Vec<serde_json::Value>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
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
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::CommitmentsScalarColumnType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "values" => {
                            values = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;
                let values = values.ok_or_else(|| M::Error::missing_field("values"))?;

                let content = CommitmentsScalarColumn {
                    meta,
                    name,
                    type_,
                    values,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CommitmentsScalarColumnVisitor)
    }
}

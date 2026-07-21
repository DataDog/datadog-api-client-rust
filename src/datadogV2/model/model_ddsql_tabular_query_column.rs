// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A single column of a DDSQL tabular query result.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DdsqlTabularQueryColumn {
    /// Name of the column as projected by the SQL statement.
    #[serde(rename = "name")]
    pub name: String,
    /// DDSQL data type of the column's values, for example `VARCHAR`, `BIGINT`,
    /// `DECIMAL`, `BOOLEAN`, `TIMESTAMP`, `JSON`, or an array variant such as
    /// `VARCHAR[]`. See the
    /// [DDSQL data-types reference](<https://docs.datadoghq.com/ddsql_reference/#data-types>)
    /// for the full, up-to-date list.
    #[serde(rename = "type")]
    pub type_: String,
    /// Column values in row order. The element type matches the column's `type`;
    /// for example a `VARCHAR` column carries strings, a `TIMESTAMP` column carries
    /// Unix-millisecond integers. `null` is allowed for missing values.
    #[serde(rename = "values")]
    pub values: Vec<serde_json::Value>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DdsqlTabularQueryColumn {
    pub fn new(
        name: String,
        type_: String,
        values: Vec<serde_json::Value>,
    ) -> DdsqlTabularQueryColumn {
        DdsqlTabularQueryColumn {
            name,
            type_,
            values,
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

impl<'de> Deserialize<'de> for DdsqlTabularQueryColumn {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DdsqlTabularQueryColumnVisitor;
        impl<'a> Visitor<'a> for DdsqlTabularQueryColumnVisitor {
            type Value = DdsqlTabularQueryColumn;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut name: Option<String> = None;
                let mut type_: Option<String> = None;
                let mut values: Option<Vec<serde_json::Value>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = DdsqlTabularQueryColumn {
                    name,
                    type_,
                    values,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DdsqlTabularQueryColumnVisitor)
    }
}

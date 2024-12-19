// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Static library vulnerability location.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DependencyLocation {
    /// Location column end.
    #[serde(rename = "column_end")]
    pub column_end: i64,
    /// Location column start.
    #[serde(rename = "column_start")]
    pub column_start: i64,
    /// Location file name.
    #[serde(rename = "file_name")]
    pub file_name: String,
    /// Location line end.
    #[serde(rename = "line_end")]
    pub line_end: i64,
    /// Location line start.
    #[serde(rename = "line_start")]
    pub line_start: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DependencyLocation {
    pub fn new(
        column_end: i64,
        column_start: i64,
        file_name: String,
        line_end: i64,
        line_start: i64,
    ) -> DependencyLocation {
        DependencyLocation {
            column_end,
            column_start,
            file_name,
            line_end,
            line_start,
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

impl<'de> Deserialize<'de> for DependencyLocation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DependencyLocationVisitor;
        impl<'a> Visitor<'a> for DependencyLocationVisitor {
            type Value = DependencyLocation;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut column_end: Option<i64> = None;
                let mut column_start: Option<i64> = None;
                let mut file_name: Option<String> = None;
                let mut line_end: Option<i64> = None;
                let mut line_start: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "column_end" => {
                            column_end = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "column_start" => {
                            column_start =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "file_name" => {
                            file_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "line_end" => {
                            line_end = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "line_start" => {
                            line_start = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let column_end = column_end.ok_or_else(|| M::Error::missing_field("column_end"))?;
                let column_start =
                    column_start.ok_or_else(|| M::Error::missing_field("column_start"))?;
                let file_name = file_name.ok_or_else(|| M::Error::missing_field("file_name"))?;
                let line_end = line_end.ok_or_else(|| M::Error::missing_field("line_end"))?;
                let line_start = line_start.ok_or_else(|| M::Error::missing_field("line_start"))?;

                let content = DependencyLocation {
                    column_end,
                    column_start,
                    file_name,
                    line_end,
                    line_start,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DependencyLocationVisitor)
    }
}

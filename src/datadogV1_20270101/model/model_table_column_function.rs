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
pub struct TableColumnFunction {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "args")]
    pub args: Option<Vec<crate::datadogV1_20270101::model::DashboardInlineSchema47>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl TableColumnFunction {
    pub fn new(name: String) -> TableColumnFunction {
        TableColumnFunction {
            args: None,
            name,
            _unparsed: false,
        }
    }
    
    pub fn args(mut self, value: Vec<crate::datadogV1_20270101::model::DashboardInlineSchema47>) -> Self {
        self.args = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for TableColumnFunction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TableColumnFunctionVisitor;
        impl<'a> Visitor<'a> for TableColumnFunctionVisitor {
            type Value = TableColumnFunction;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut args: Option<Vec<crate::datadogV1_20270101::model::DashboardInlineSchema47>> = None;
                let mut name: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "args" => {
                            if v.is_null() {
                                continue;
                            }
                            args = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = TableColumnFunction {
                    args,
                    name,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TableColumnFunctionVisitor)
    }
}

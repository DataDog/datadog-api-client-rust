// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Column values for this row in the reference table.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TableRowResourceDataAttributes {
    /// Key-value pairs representing the row data, where keys are field names from the schema.
    #[serde(rename = "values")]
    pub values: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TableRowResourceDataAttributes {
    pub fn new() -> TableRowResourceDataAttributes {
        TableRowResourceDataAttributes {
            values: None,
            _unparsed: false,
        }
    }

    pub fn values(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.values = Some(value);
        self
    }
}

impl Default for TableRowResourceDataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TableRowResourceDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TableRowResourceDataAttributesVisitor;
        impl<'a> Visitor<'a> for TableRowResourceDataAttributesVisitor {
            type Value = TableRowResourceDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut values: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "values" => {
                            if v.is_null() {
                                continue;
                            }
                            values = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }

                let content = TableRowResourceDataAttributes { values, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TableRowResourceDataAttributesVisitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Widget column.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ListStreamColumn {
    /// Widget column field.
    #[serde(rename = "field")]
    pub field: String,
    /// Widget column width.
    #[serde(rename = "width")]
    pub width: crate::datadogV1::model::ListStreamColumnWidth,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ListStreamColumn {
    pub fn new(
        field: String,
        width: crate::datadogV1::model::ListStreamColumnWidth,
    ) -> ListStreamColumn {
        ListStreamColumn {
            field,
            width,
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

impl<'de> Deserialize<'de> for ListStreamColumn {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ListStreamColumnVisitor;
        impl<'a> Visitor<'a> for ListStreamColumnVisitor {
            type Value = ListStreamColumn;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut field: Option<String> = None;
                let mut width: Option<crate::datadogV1::model::ListStreamColumnWidth> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "field" => {
                            field = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "width" => {
                            width = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _width) = width {
                                match _width {
                                    crate::datadogV1::model::ListStreamColumnWidth::UnparsedObject(_width) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let field = field.ok_or_else(|| M::Error::missing_field("field"))?;
                let width = width.ok_or_else(|| M::Error::missing_field("width"))?;

                let content = ListStreamColumn {
                    field,
                    width,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ListStreamColumnVisitor)
    }
}

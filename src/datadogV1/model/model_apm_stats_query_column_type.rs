// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Column properties.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ApmStatsQueryColumnType {
    /// A user-assigned alias for the column.
    #[serde(rename = "alias")]
    pub alias: Option<String>,
    /// Define a display mode for the table cell.
    #[serde(rename = "cell_display_mode")]
    pub cell_display_mode: Option<crate::datadogV1::model::TableWidgetCellDisplayMode>,
    /// Column name.
    #[serde(rename = "name")]
    pub name: String,
    /// Widget sorting methods.
    #[serde(rename = "order")]
    pub order: Option<crate::datadogV1::model::WidgetSort>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ApmStatsQueryColumnType {
    pub fn new(name: String) -> ApmStatsQueryColumnType {
        ApmStatsQueryColumnType {
            alias: None,
            cell_display_mode: None,
            name,
            order: None,
            _unparsed: false,
        }
    }

    pub fn alias(mut self, value: String) -> Self {
        self.alias = Some(value);
        self
    }

    pub fn cell_display_mode(
        mut self,
        value: crate::datadogV1::model::TableWidgetCellDisplayMode,
    ) -> Self {
        self.cell_display_mode = Some(value);
        self
    }

    pub fn order(mut self, value: crate::datadogV1::model::WidgetSort) -> Self {
        self.order = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for ApmStatsQueryColumnType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ApmStatsQueryColumnTypeVisitor;
        impl<'a> Visitor<'a> for ApmStatsQueryColumnTypeVisitor {
            type Value = ApmStatsQueryColumnType;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut alias: Option<String> = None;
                let mut cell_display_mode: Option<
                    crate::datadogV1::model::TableWidgetCellDisplayMode,
                > = None;
                let mut name: Option<String> = None;
                let mut order: Option<crate::datadogV1::model::WidgetSort> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "alias" => {
                            if v.is_null() {
                                continue;
                            }
                            alias = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cell_display_mode" => {
                            if v.is_null() {
                                continue;
                            }
                            cell_display_mode =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _cell_display_mode) = cell_display_mode {
                                match _cell_display_mode {
                                    crate::datadogV1::model::TableWidgetCellDisplayMode::UnparsedObject(_cell_display_mode) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "order" => {
                            if v.is_null() {
                                continue;
                            }
                            order = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _order) = order {
                                match _order {
                                    crate::datadogV1::model::WidgetSort::UnparsedObject(_order) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = ApmStatsQueryColumnType {
                    alias,
                    cell_display_mode,
                    name,
                    order,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ApmStatsQueryColumnTypeVisitor)
    }
}

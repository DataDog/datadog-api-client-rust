// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Dimension mapping for the point plot projection.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PointPlotProjectionDimension {
    /// Alias for the column.
    #[serde(rename = "alias")]
    pub alias: Option<String>,
    /// Source column name from the dataset.
    #[serde(rename = "column")]
    pub column: String,
    /// Dimension of the point plot.
    #[serde(rename = "dimension")]
    pub dimension: crate::datadogV1::model::PointPlotDimension,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl PointPlotProjectionDimension {
    pub fn new(
        column: String,
        dimension: crate::datadogV1::model::PointPlotDimension,
    ) -> PointPlotProjectionDimension {
        PointPlotProjectionDimension {
            alias: None,
            column,
            dimension,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn alias(mut self, value: String) -> Self {
        self.alias = Some(value);
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

impl<'de> Deserialize<'de> for PointPlotProjectionDimension {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PointPlotProjectionDimensionVisitor;
        impl<'a> Visitor<'a> for PointPlotProjectionDimensionVisitor {
            type Value = PointPlotProjectionDimension;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut alias: Option<String> = None;
                let mut column: Option<String> = None;
                let mut dimension: Option<crate::datadogV1::model::PointPlotDimension> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "alias" => {
                            if v.is_null() {
                                continue;
                            }
                            alias = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "column" => {
                            column = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "dimension" => {
                            dimension = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _dimension) = dimension {
                                match _dimension {
                                    crate::datadogV1::model::PointPlotDimension::UnparsedObject(
                                        _dimension,
                                    ) => {
                                        _unparsed = true;
                                    }
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
                let column = column.ok_or_else(|| M::Error::missing_field("column"))?;
                let dimension = dimension.ok_or_else(|| M::Error::missing_field("dimension"))?;

                let content = PointPlotProjectionDimension {
                    alias,
                    column,
                    dimension,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(PointPlotProjectionDimensionVisitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A single dimension mapping for a scatterplot data projection.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ScatterplotDataProjectionDimension {
    /// Display alias for the dimension.
    #[serde(rename = "alias")]
    pub alias: Option<String>,
    /// The column name from the data source.
    #[serde(rename = "column")]
    pub column: String,
    /// Dimension of the Scatterplot.
    #[serde(rename = "dimension")]
    pub dimension: crate::datadogV1::model::ScatterplotDimension,
    /// Number format options for the widget.
    #[serde(rename = "number_format")]
    pub number_format: Option<crate::datadogV1::model::WidgetNumberFormat>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ScatterplotDataProjectionDimension {
    pub fn new(
        column: String,
        dimension: crate::datadogV1::model::ScatterplotDimension,
    ) -> ScatterplotDataProjectionDimension {
        ScatterplotDataProjectionDimension {
            alias: None,
            column,
            dimension,
            number_format: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn alias(mut self, value: String) -> Self {
        self.alias = Some(value);
        self
    }

    pub fn number_format(mut self, value: crate::datadogV1::model::WidgetNumberFormat) -> Self {
        self.number_format = Some(value);
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

impl<'de> Deserialize<'de> for ScatterplotDataProjectionDimension {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ScatterplotDataProjectionDimensionVisitor;
        impl<'a> Visitor<'a> for ScatterplotDataProjectionDimensionVisitor {
            type Value = ScatterplotDataProjectionDimension;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut alias: Option<String> = None;
                let mut column: Option<String> = None;
                let mut dimension: Option<crate::datadogV1::model::ScatterplotDimension> = None;
                let mut number_format: Option<crate::datadogV1::model::WidgetNumberFormat> = None;
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
                                    crate::datadogV1::model::ScatterplotDimension::UnparsedObject(_dimension) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "number_format" => {
                            if v.is_null() {
                                continue;
                            }
                            number_format =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = ScatterplotDataProjectionDimension {
                    alias,
                    column,
                    dimension,
                    number_format,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ScatterplotDataProjectionDimensionVisitor)
    }
}

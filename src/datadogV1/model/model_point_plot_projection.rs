// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Projection configuration for the point plot widget.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PointPlotProjection {
    /// List of dimension mappings for the projection.
    #[serde(rename = "dimensions")]
    pub dimensions: Vec<crate::datadogV1::model::PointPlotProjectionDimension>,
    /// Additional columns to include in the projection.
    #[serde(rename = "extra_columns")]
    pub extra_columns: Option<Vec<String>>,
    /// Type of the projection.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::PointPlotProjectionType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl PointPlotProjection {
    pub fn new(
        dimensions: Vec<crate::datadogV1::model::PointPlotProjectionDimension>,
        type_: crate::datadogV1::model::PointPlotProjectionType,
    ) -> PointPlotProjection {
        PointPlotProjection {
            dimensions,
            extra_columns: None,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn extra_columns(mut self, value: Vec<String>) -> Self {
        self.extra_columns = Some(value);
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

impl<'de> Deserialize<'de> for PointPlotProjection {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PointPlotProjectionVisitor;
        impl<'a> Visitor<'a> for PointPlotProjectionVisitor {
            type Value = PointPlotProjection;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut dimensions: Option<
                    Vec<crate::datadogV1::model::PointPlotProjectionDimension>,
                > = None;
                let mut extra_columns: Option<Vec<String>> = None;
                let mut type_: Option<crate::datadogV1::model::PointPlotProjectionType> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "dimensions" => {
                            dimensions = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "extra_columns" => {
                            if v.is_null() {
                                continue;
                            }
                            extra_columns =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::PointPlotProjectionType::UnparsedObject(_type_) => {
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
                let dimensions = dimensions.ok_or_else(|| M::Error::missing_field("dimensions"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = PointPlotProjection {
                    dimensions,
                    extra_columns,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(PointPlotProjectionVisitor)
    }
}

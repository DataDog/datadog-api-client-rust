// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Request configuration for the point plot widget.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PointPlotWidgetRequest {
    /// Maximum number of data points to return.
    #[serde(rename = "limit")]
    pub limit: Option<i64>,
    /// Projection configuration for the point plot widget.
    #[serde(rename = "projection")]
    pub projection: crate::datadogV1::model::PointPlotProjection,
    /// Query configuration for a data projection request.
    #[serde(rename = "query")]
    pub query: crate::datadogV1::model::DataProjectionQuery,
    /// Type of a data projection request.
    #[serde(rename = "request_type")]
    pub request_type: crate::datadogV1::model::DataProjectionRequestType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl PointPlotWidgetRequest {
    pub fn new(
        projection: crate::datadogV1::model::PointPlotProjection,
        query: crate::datadogV1::model::DataProjectionQuery,
        request_type: crate::datadogV1::model::DataProjectionRequestType,
    ) -> PointPlotWidgetRequest {
        PointPlotWidgetRequest {
            limit: None,
            projection,
            query,
            request_type,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
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

impl<'de> Deserialize<'de> for PointPlotWidgetRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PointPlotWidgetRequestVisitor;
        impl<'a> Visitor<'a> for PointPlotWidgetRequestVisitor {
            type Value = PointPlotWidgetRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut limit: Option<i64> = None;
                let mut projection: Option<crate::datadogV1::model::PointPlotProjection> = None;
                let mut query: Option<crate::datadogV1::model::DataProjectionQuery> = None;
                let mut request_type: Option<crate::datadogV1::model::DataProjectionRequestType> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "limit" => {
                            if v.is_null() {
                                continue;
                            }
                            limit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "projection" => {
                            projection = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query" => {
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "request_type" => {
                            request_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _request_type) = request_type {
                                match _request_type {
                                    crate::datadogV1::model::DataProjectionRequestType::UnparsedObject(_request_type) => {
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
                let projection = projection.ok_or_else(|| M::Error::missing_field("projection"))?;
                let query = query.ok_or_else(|| M::Error::missing_field("query"))?;
                let request_type =
                    request_type.ok_or_else(|| M::Error::missing_field("request_type"))?;

                let content = PointPlotWidgetRequest {
                    limit,
                    projection,
                    query,
                    request_type,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(PointPlotWidgetRequestVisitor)
    }
}

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
pub struct TimeseriesWidgetLocalDatasetRequest {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "display_type")]
    pub display_type: Option<crate::datadogV1_20270101::model::WidgetDisplayType>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "metadata")]
    pub metadata: Option<Vec<crate::datadogV1_20270101::model::TimeseriesWidgetExpressionAlias>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "on_right_yaxis")]
    pub on_right_yaxis: Option<bool>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "projection")]
    pub projection: Option<crate::datadogV1_20270101::model::TimeseriesProjection>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "query")]
    pub query: crate::datadogV1_20270101::model::StructuredAnalysisQuery,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "request_type")]
    pub request_type: crate::datadogV1_20270101::model::TimeseriesWidgetLocalDatasetRequestRequestType,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "response_format")]
    pub response_format: crate::datadogV1_20270101::model::TimeseriesWidgetLocalDatasetRequestResponseFormat,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "style")]
    pub style: Option<crate::datadogV1_20270101::model::TimeseriesRequestStyle>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl TimeseriesWidgetLocalDatasetRequest {
    pub fn new(query: crate::datadogV1_20270101::model::StructuredAnalysisQuery, request_type: crate::datadogV1_20270101::model::TimeseriesWidgetLocalDatasetRequestRequestType, response_format: crate::datadogV1_20270101::model::TimeseriesWidgetLocalDatasetRequestResponseFormat) -> TimeseriesWidgetLocalDatasetRequest {
        TimeseriesWidgetLocalDatasetRequest {
            display_type: None,
            metadata: None,
            on_right_yaxis: None,
            projection: None,
            query,
            request_type,
            response_format,
            style: None,
            _unparsed: false,
        }
    }
    
    pub fn display_type(mut self, value: crate::datadogV1_20270101::model::WidgetDisplayType) -> Self {
        self.display_type = Some(value);
        self
    }
    
    pub fn metadata(mut self, value: Vec<crate::datadogV1_20270101::model::TimeseriesWidgetExpressionAlias>) -> Self {
        self.metadata = Some(value);
        self
    }
    
    pub fn on_right_yaxis(mut self, value: bool) -> Self {
        self.on_right_yaxis = Some(value);
        self
    }
    
    pub fn projection(mut self, value: crate::datadogV1_20270101::model::TimeseriesProjection) -> Self {
        self.projection = Some(value);
        self
    }
    
    pub fn style(mut self, value: crate::datadogV1_20270101::model::TimeseriesRequestStyle) -> Self {
        self.style = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for TimeseriesWidgetLocalDatasetRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TimeseriesWidgetLocalDatasetRequestVisitor;
        impl<'a> Visitor<'a> for TimeseriesWidgetLocalDatasetRequestVisitor {
            type Value = TimeseriesWidgetLocalDatasetRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut display_type: Option<crate::datadogV1_20270101::model::WidgetDisplayType> = None;
                let mut metadata: Option<Vec<crate::datadogV1_20270101::model::TimeseriesWidgetExpressionAlias>> = None;
                let mut on_right_yaxis: Option<bool> = None;
                let mut projection: Option<crate::datadogV1_20270101::model::TimeseriesProjection> = None;
                let mut query: Option<crate::datadogV1_20270101::model::StructuredAnalysisQuery> = None;
                let mut request_type: Option<crate::datadogV1_20270101::model::TimeseriesWidgetLocalDatasetRequestRequestType> = None;
                let mut response_format: Option<crate::datadogV1_20270101::model::TimeseriesWidgetLocalDatasetRequestResponseFormat> = None;
                let mut style: Option<crate::datadogV1_20270101::model::TimeseriesRequestStyle> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "display_type" => {
                            if v.is_null() {
                                continue;
                            }
                            display_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _display_type) = display_type {
                                match _display_type {
                                    crate::datadogV1_20270101::model::WidgetDisplayType::UnparsedObject(_display_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "metadata" => {
                            if v.is_null() {
                                continue;
                            }
                            metadata = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "on_right_yaxis" => {
                            if v.is_null() {
                                continue;
                            }
                            on_right_yaxis = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "projection" => {
                            if v.is_null() {
                                continue;
                            }
                            projection = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "query" => {
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "request_type" => {
                            request_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _request_type) = request_type {
                                match _request_type {
                                    crate::datadogV1_20270101::model::TimeseriesWidgetLocalDatasetRequestRequestType::UnparsedObject(_request_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "response_format" => {
                            response_format = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _response_format) = response_format {
                                match _response_format {
                                    crate::datadogV1_20270101::model::TimeseriesWidgetLocalDatasetRequestResponseFormat::UnparsedObject(_response_format) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "style" => {
                            if v.is_null() {
                                continue;
                            }
                            style = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let query = query.ok_or_else(|| M::Error::missing_field("query"))?;
                let request_type = request_type.ok_or_else(|| M::Error::missing_field("request_type"))?;
                let response_format = response_format.ok_or_else(|| M::Error::missing_field("response_format"))?;

                let content = TimeseriesWidgetLocalDatasetRequest {
                    display_type,
                    metadata,
                    on_right_yaxis,
                    projection,
                    query,
                    request_type,
                    response_format,
                    style,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TimeseriesWidgetLocalDatasetRequestVisitor)
    }
}

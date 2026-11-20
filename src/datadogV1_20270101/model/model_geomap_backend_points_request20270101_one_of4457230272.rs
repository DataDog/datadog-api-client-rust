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
pub struct GeomapBackendPointsRequest20270101OneOf4457230272 {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "conditional_formats")]
    pub conditional_formats: Option<Vec<crate::datadogV1_20270101::model::ThresholdFormattingRule>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "limit")]
    pub limit: Option<f64>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "projection")]
    pub projection: crate::datadogV1_20270101::model::GeomapPointsProjection,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "query")]
    pub query: crate::datadogV1_20270101::model::DataProjectionQuery,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "request_type")]
    pub request_type: crate::datadogV1_20270101::model::GeomapBackendPointsRequestAnyOfAnyOfItemRequestType4447003616,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "style")]
    pub style: Option<crate::datadogV1_20270101::model::GeomapBackendPointsRequestAnyOfAnyOfItemStyle>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "text_formats")]
    pub text_formats: Option<Vec<crate::datadogV1_20270101::model::TextFormattingRule>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl GeomapBackendPointsRequest20270101OneOf4457230272 {
    pub fn new(projection: crate::datadogV1_20270101::model::GeomapPointsProjection, query: crate::datadogV1_20270101::model::DataProjectionQuery, request_type: crate::datadogV1_20270101::model::GeomapBackendPointsRequestAnyOfAnyOfItemRequestType4447003616) -> GeomapBackendPointsRequest20270101OneOf4457230272 {
        GeomapBackendPointsRequest20270101OneOf4457230272 {
            conditional_formats: None,
            limit: None,
            projection,
            query,
            request_type,
            style: None,
            text_formats: None,
            _unparsed: false,
        }
    }
    
    pub fn conditional_formats(mut self, value: Vec<crate::datadogV1_20270101::model::ThresholdFormattingRule>) -> Self {
        self.conditional_formats = Some(value);
        self
    }
    
    pub fn limit(mut self, value: f64) -> Self {
        self.limit = Some(value);
        self
    }
    
    pub fn style(mut self, value: crate::datadogV1_20270101::model::GeomapBackendPointsRequestAnyOfAnyOfItemStyle) -> Self {
        self.style = Some(value);
        self
    }
    
    pub fn text_formats(mut self, value: Vec<crate::datadogV1_20270101::model::TextFormattingRule>) -> Self {
        self.text_formats = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for GeomapBackendPointsRequest20270101OneOf4457230272 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GeomapBackendPointsRequest20270101OneOf4457230272Visitor;
        impl<'a> Visitor<'a> for GeomapBackendPointsRequest20270101OneOf4457230272Visitor {
            type Value = GeomapBackendPointsRequest20270101OneOf4457230272;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut conditional_formats: Option<Vec<crate::datadogV1_20270101::model::ThresholdFormattingRule>> = None;
                let mut limit: Option<f64> = None;
                let mut projection: Option<crate::datadogV1_20270101::model::GeomapPointsProjection> = None;
                let mut query: Option<crate::datadogV1_20270101::model::DataProjectionQuery> = None;
                let mut request_type: Option<crate::datadogV1_20270101::model::GeomapBackendPointsRequestAnyOfAnyOfItemRequestType4447003616> = None;
                let mut style: Option<crate::datadogV1_20270101::model::GeomapBackendPointsRequestAnyOfAnyOfItemStyle> = None;
                let mut text_formats: Option<Vec<crate::datadogV1_20270101::model::TextFormattingRule>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "conditional_formats" => {
                            if v.is_null() {
                                continue;
                            }
                            conditional_formats = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "limit" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            limit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "projection" => {
                            projection = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "query" => {
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _query) = query {
                                match _query {
                                    crate::datadogV1_20270101::model::DataProjectionQuery::UnparsedObject(_query) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "request_type" => {
                            request_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _request_type) = request_type {
                                match _request_type {
                                    crate::datadogV1_20270101::model::GeomapBackendPointsRequestAnyOfAnyOfItemRequestType4447003616::UnparsedObject(_request_type) => {
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
                        "text_formats" => {
                            if v.is_null() {
                                continue;
                            }
                            text_formats = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let projection = projection.ok_or_else(|| M::Error::missing_field("projection"))?;
                let query = query.ok_or_else(|| M::Error::missing_field("query"))?;
                let request_type = request_type.ok_or_else(|| M::Error::missing_field("request_type"))?;

                let content = GeomapBackendPointsRequest20270101OneOf4457230272 {
                    conditional_formats,
                    limit,
                    projection,
                    query,
                    request_type,
                    style,
                    text_formats,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GeomapBackendPointsRequest20270101OneOf4457230272Visitor)
    }
}

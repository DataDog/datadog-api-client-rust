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
pub struct GeomapBackendPointsRequest20270101OneOf4457228992 {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "query")]
    pub query: crate::datadogV1_20270101::model::GeomapBackendPointsRequestAnyOfAnyOfItemQuery,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "request_type")]
    pub request_type: crate::datadogV1_20270101::model::GeomapBackendPointsRequestAnyOfAnyOfItemRequestType,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "response_format")]
    pub response_format: crate::datadogV1_20270101::model::GeomapBackendPointsRequestAnyOfAnyOfItemResponseFormat4447001376,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "style")]
    pub style: Option<crate::datadogV1_20270101::model::GeomapRequestStyle>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "text_formats")]
    pub text_formats: Option<Vec<crate::datadogV1_20270101::model::TextFormattingRule>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl GeomapBackendPointsRequest20270101OneOf4457228992 {
    pub fn new(query: crate::datadogV1_20270101::model::GeomapBackendPointsRequestAnyOfAnyOfItemQuery, request_type: crate::datadogV1_20270101::model::GeomapBackendPointsRequestAnyOfAnyOfItemRequestType, response_format: crate::datadogV1_20270101::model::GeomapBackendPointsRequestAnyOfAnyOfItemResponseFormat4447001376) -> GeomapBackendPointsRequest20270101OneOf4457228992 {
        GeomapBackendPointsRequest20270101OneOf4457228992 {
            query,
            request_type,
            response_format,
            style: None,
            text_formats: None,
            _unparsed: false,
        }
    }
    
    pub fn style(mut self, value: crate::datadogV1_20270101::model::GeomapRequestStyle) -> Self {
        self.style = Some(value);
        self
    }
    
    pub fn text_formats(mut self, value: Vec<crate::datadogV1_20270101::model::TextFormattingRule>) -> Self {
        self.text_formats = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for GeomapBackendPointsRequest20270101OneOf4457228992 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GeomapBackendPointsRequest20270101OneOf4457228992Visitor;
        impl<'a> Visitor<'a> for GeomapBackendPointsRequest20270101OneOf4457228992Visitor {
            type Value = GeomapBackendPointsRequest20270101OneOf4457228992;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut query: Option<crate::datadogV1_20270101::model::GeomapBackendPointsRequestAnyOfAnyOfItemQuery> = None;
                let mut request_type: Option<crate::datadogV1_20270101::model::GeomapBackendPointsRequestAnyOfAnyOfItemRequestType> = None;
                let mut response_format: Option<crate::datadogV1_20270101::model::GeomapBackendPointsRequestAnyOfAnyOfItemResponseFormat4447001376> = None;
                let mut style: Option<crate::datadogV1_20270101::model::GeomapRequestStyle> = None;
                let mut text_formats: Option<Vec<crate::datadogV1_20270101::model::TextFormattingRule>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "query" => {
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "request_type" => {
                            request_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _request_type) = request_type {
                                match _request_type {
                                    crate::datadogV1_20270101::model::GeomapBackendPointsRequestAnyOfAnyOfItemRequestType::UnparsedObject(_request_type) => {
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
                                    crate::datadogV1_20270101::model::GeomapBackendPointsRequestAnyOfAnyOfItemResponseFormat4447001376::UnparsedObject(_response_format) => {
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
                let query = query.ok_or_else(|| M::Error::missing_field("query"))?;
                let request_type = request_type.ok_or_else(|| M::Error::missing_field("request_type"))?;
                let response_format = response_format.ok_or_else(|| M::Error::missing_field("response_format"))?;

                let content = GeomapBackendPointsRequest20270101OneOf4457228992 {
                    query,
                    request_type,
                    response_format,
                    style,
                    text_formats,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GeomapBackendPointsRequest20270101OneOf4457228992Visitor)
    }
}

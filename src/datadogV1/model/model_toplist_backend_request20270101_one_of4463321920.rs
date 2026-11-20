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
pub struct ToplistBackendRequest20270101OneOf4463321920 {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "conditional_formats")]
    pub conditional_formats: Option<Vec<crate::datadogV1::model::ThresholdFormattingRule>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "projection")]
    pub projection: Option<crate::datadogV1::model::TabularProjection>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "query")]
    pub query: crate::datadogV1::model::ToplistBackendRequest20270101OneOfQuery,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "request_type")]
    pub request_type: crate::datadogV1::model::ToplistBackendRequestAnyOfAnyOfItemRequestType4453036448,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "response_format")]
    pub response_format: crate::datadogV1::model::ToplistBackendRequestAnyOfAnyOfItemResponseFormat4453036768,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ToplistBackendRequest20270101OneOf4463321920 {
    pub fn new(query: crate::datadogV1::model::ToplistBackendRequest20270101OneOfQuery, request_type: crate::datadogV1::model::ToplistBackendRequestAnyOfAnyOfItemRequestType4453036448, response_format: crate::datadogV1::model::ToplistBackendRequestAnyOfAnyOfItemResponseFormat4453036768) -> ToplistBackendRequest20270101OneOf4463321920 {
        ToplistBackendRequest20270101OneOf4463321920 {
            conditional_formats: None,
            projection: None,
            query,
            request_type,
            response_format,
            _unparsed: false,
        }
    }
    
    pub fn conditional_formats(mut self, value: Vec<crate::datadogV1::model::ThresholdFormattingRule>) -> Self {
        self.conditional_formats = Some(value);
        self
    }
    
    pub fn projection(mut self, value: crate::datadogV1::model::TabularProjection) -> Self {
        self.projection = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for ToplistBackendRequest20270101OneOf4463321920 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ToplistBackendRequest20270101OneOf4463321920Visitor;
        impl<'a> Visitor<'a> for ToplistBackendRequest20270101OneOf4463321920Visitor {
            type Value = ToplistBackendRequest20270101OneOf4463321920;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut conditional_formats: Option<Vec<crate::datadogV1::model::ThresholdFormattingRule>> = None;
                let mut projection: Option<crate::datadogV1::model::TabularProjection> = None;
                let mut query: Option<crate::datadogV1::model::ToplistBackendRequest20270101OneOfQuery> = None;
                let mut request_type: Option<crate::datadogV1::model::ToplistBackendRequestAnyOfAnyOfItemRequestType4453036448> = None;
                let mut response_format: Option<crate::datadogV1::model::ToplistBackendRequestAnyOfAnyOfItemResponseFormat4453036768> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "conditional_formats" => {
                            if v.is_null() {
                                continue;
                            }
                            conditional_formats = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "projection" => {
                            if v.is_null() {
                                continue;
                            }
                            projection = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _projection) = projection {
                                match _projection {
                                    crate::datadogV1::model::TabularProjection::UnparsedObject(_projection) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "query" => {
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _query) = query {
                                match _query {
                                    crate::datadogV1::model::ToplistBackendRequest20270101OneOfQuery::UnparsedObject(_query) => {
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
                                    crate::datadogV1::model::ToplistBackendRequestAnyOfAnyOfItemRequestType4453036448::UnparsedObject(_request_type) => {
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
                                    crate::datadogV1::model::ToplistBackendRequestAnyOfAnyOfItemResponseFormat4453036768::UnparsedObject(_response_format) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let query = query.ok_or_else(|| M::Error::missing_field("query"))?;
                let request_type = request_type.ok_or_else(|| M::Error::missing_field("request_type"))?;
                let response_format = response_format.ok_or_else(|| M::Error::missing_field("response_format"))?;

                let content = ToplistBackendRequest20270101OneOf4463321920 {
                    conditional_formats,
                    projection,
                    query,
                    request_type,
                    response_format,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ToplistBackendRequest20270101OneOf4463321920Visitor)
    }
}

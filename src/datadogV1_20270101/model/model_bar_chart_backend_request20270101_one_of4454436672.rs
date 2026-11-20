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
pub struct BarChartBackendRequest20270101OneOf4454436672 {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "conditional_formats")]
    pub conditional_formats: Option<Vec<crate::datadogV1_20270101::model::ThresholdFormattingRule>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "projection")]
    pub projection: Option<crate::datadogV1_20270101::model::TabularProjection>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "query")]
    pub query: crate::datadogV1_20270101::model::BarChartBackendRequest20270101OneOfQuery,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "request_type")]
    pub request_type: crate::datadogV1_20270101::model::BarChartBackendRequestAnyOfAnyOfItemRequestType4443924544,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "response_format")]
    pub response_format: crate::datadogV1_20270101::model::BarChartBackendRequestAnyOfAnyOfItemResponseFormat4443924864,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl BarChartBackendRequest20270101OneOf4454436672 {
    pub fn new(query: crate::datadogV1_20270101::model::BarChartBackendRequest20270101OneOfQuery, request_type: crate::datadogV1_20270101::model::BarChartBackendRequestAnyOfAnyOfItemRequestType4443924544, response_format: crate::datadogV1_20270101::model::BarChartBackendRequestAnyOfAnyOfItemResponseFormat4443924864) -> BarChartBackendRequest20270101OneOf4454436672 {
        BarChartBackendRequest20270101OneOf4454436672 {
            conditional_formats: None,
            projection: None,
            query,
            request_type,
            response_format,
            _unparsed: false,
        }
    }
    
    pub fn conditional_formats(mut self, value: Vec<crate::datadogV1_20270101::model::ThresholdFormattingRule>) -> Self {
        self.conditional_formats = Some(value);
        self
    }
    
    pub fn projection(mut self, value: crate::datadogV1_20270101::model::TabularProjection) -> Self {
        self.projection = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for BarChartBackendRequest20270101OneOf4454436672 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BarChartBackendRequest20270101OneOf4454436672Visitor;
        impl<'a> Visitor<'a> for BarChartBackendRequest20270101OneOf4454436672Visitor {
            type Value = BarChartBackendRequest20270101OneOf4454436672;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut conditional_formats: Option<Vec<crate::datadogV1_20270101::model::ThresholdFormattingRule>> = None;
                let mut projection: Option<crate::datadogV1_20270101::model::TabularProjection> = None;
                let mut query: Option<crate::datadogV1_20270101::model::BarChartBackendRequest20270101OneOfQuery> = None;
                let mut request_type: Option<crate::datadogV1_20270101::model::BarChartBackendRequestAnyOfAnyOfItemRequestType4443924544> = None;
                let mut response_format: Option<crate::datadogV1_20270101::model::BarChartBackendRequestAnyOfAnyOfItemResponseFormat4443924864> = None;
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
                                    crate::datadogV1_20270101::model::TabularProjection::UnparsedObject(_projection) => {
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
                                    crate::datadogV1_20270101::model::BarChartBackendRequest20270101OneOfQuery::UnparsedObject(_query) => {
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
                                    crate::datadogV1_20270101::model::BarChartBackendRequestAnyOfAnyOfItemRequestType4443924544::UnparsedObject(_request_type) => {
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
                                    crate::datadogV1_20270101::model::BarChartBackendRequestAnyOfAnyOfItemResponseFormat4443924864::UnparsedObject(_response_format) => {
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

                let content = BarChartBackendRequest20270101OneOf4454436672 {
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

        deserializer.deserialize_any(BarChartBackendRequest20270101OneOf4454436672Visitor)
    }
}

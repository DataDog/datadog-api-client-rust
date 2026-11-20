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
pub struct ScatterplotBackendTableRequest20270101OneOf4461521920 {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "projection")]
    pub projection: Option<crate::datadogV1::model::TabularProjection>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "query")]
    pub query: crate::datadogV1::model::ScatterplotBackendTableRequest20270101OneOfQuery,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "request_type")]
    pub request_type: crate::datadogV1::model::ScatterplotBackendTableRequestAnyOfAnyOfItemRequestType4451204320,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "response_format")]
    pub response_format: crate::datadogV1::model::ScatterplotBackendTableRequestAnyOfAnyOfItemResponseFormat4451204640,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ScatterplotBackendTableRequest20270101OneOf4461521920 {
    pub fn new(query: crate::datadogV1::model::ScatterplotBackendTableRequest20270101OneOfQuery, request_type: crate::datadogV1::model::ScatterplotBackendTableRequestAnyOfAnyOfItemRequestType4451204320, response_format: crate::datadogV1::model::ScatterplotBackendTableRequestAnyOfAnyOfItemResponseFormat4451204640) -> ScatterplotBackendTableRequest20270101OneOf4461521920 {
        ScatterplotBackendTableRequest20270101OneOf4461521920 {
            projection: None,
            query,
            request_type,
            response_format,
            _unparsed: false,
        }
    }
    
    pub fn projection(mut self, value: crate::datadogV1::model::TabularProjection) -> Self {
        self.projection = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for ScatterplotBackendTableRequest20270101OneOf4461521920 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ScatterplotBackendTableRequest20270101OneOf4461521920Visitor;
        impl<'a> Visitor<'a> for ScatterplotBackendTableRequest20270101OneOf4461521920Visitor {
            type Value = ScatterplotBackendTableRequest20270101OneOf4461521920;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut projection: Option<crate::datadogV1::model::TabularProjection> = None;
                let mut query: Option<crate::datadogV1::model::ScatterplotBackendTableRequest20270101OneOfQuery> = None;
                let mut request_type: Option<crate::datadogV1::model::ScatterplotBackendTableRequestAnyOfAnyOfItemRequestType4451204320> = None;
                let mut response_format: Option<crate::datadogV1::model::ScatterplotBackendTableRequestAnyOfAnyOfItemResponseFormat4451204640> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
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
                                    crate::datadogV1::model::ScatterplotBackendTableRequest20270101OneOfQuery::UnparsedObject(_query) => {
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
                                    crate::datadogV1::model::ScatterplotBackendTableRequestAnyOfAnyOfItemRequestType4451204320::UnparsedObject(_request_type) => {
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
                                    crate::datadogV1::model::ScatterplotBackendTableRequestAnyOfAnyOfItemResponseFormat4451204640::UnparsedObject(_response_format) => {
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

                let content = ScatterplotBackendTableRequest20270101OneOf4461521920 {
                    projection,
                    query,
                    request_type,
                    response_format,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ScatterplotBackendTableRequest20270101OneOf4461521920Visitor)
    }
}

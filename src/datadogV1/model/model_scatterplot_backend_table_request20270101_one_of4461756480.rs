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
pub struct ScatterplotBackendTableRequest20270101OneOf4461756480 {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "limit")]
    pub limit: Option<f64>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "projection")]
    pub projection: crate::datadogV1::model::ScatterplotProjection,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "query")]
    pub query: crate::datadogV1::model::EventsPlatformDataProjectionQuery,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "request_type")]
    pub request_type: crate::datadogV1::model::ScatterplotBackendTableRequestAnyOfAnyOfItemRequestType4451375104,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ScatterplotBackendTableRequest20270101OneOf4461756480 {
    pub fn new(projection: crate::datadogV1::model::ScatterplotProjection, query: crate::datadogV1::model::EventsPlatformDataProjectionQuery, request_type: crate::datadogV1::model::ScatterplotBackendTableRequestAnyOfAnyOfItemRequestType4451375104) -> ScatterplotBackendTableRequest20270101OneOf4461756480 {
        ScatterplotBackendTableRequest20270101OneOf4461756480 {
            limit: None,
            projection,
            query,
            request_type,
            _unparsed: false,
        }
    }
    
    pub fn limit(mut self, value: f64) -> Self {
        self.limit = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for ScatterplotBackendTableRequest20270101OneOf4461756480 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ScatterplotBackendTableRequest20270101OneOf4461756480Visitor;
        impl<'a> Visitor<'a> for ScatterplotBackendTableRequest20270101OneOf4461756480Visitor {
            type Value = ScatterplotBackendTableRequest20270101OneOf4461756480;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut limit: Option<f64> = None;
                let mut projection: Option<crate::datadogV1::model::ScatterplotProjection> = None;
                let mut query: Option<crate::datadogV1::model::EventsPlatformDataProjectionQuery> = None;
                let mut request_type: Option<crate::datadogV1::model::ScatterplotBackendTableRequestAnyOfAnyOfItemRequestType4451375104> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
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
                        },
                        "request_type" => {
                            request_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _request_type) = request_type {
                                match _request_type {
                                    crate::datadogV1::model::ScatterplotBackendTableRequestAnyOfAnyOfItemRequestType4451375104::UnparsedObject(_request_type) => {
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
                let projection = projection.ok_or_else(|| M::Error::missing_field("projection"))?;
                let query = query.ok_or_else(|| M::Error::missing_field("query"))?;
                let request_type = request_type.ok_or_else(|| M::Error::missing_field("request_type"))?;

                let content = ScatterplotBackendTableRequest20270101OneOf4461756480 {
                    limit,
                    projection,
                    query,
                    request_type,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ScatterplotBackendTableRequest20270101OneOf4461756480Visitor)
    }
}

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
pub struct PointPlotDataProjectionRequest {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "limit")]
    pub limit: Option<f64>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "projection")]
    pub projection: crate::datadogV1_20270101::model::PointPlotProjection,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "query")]
    pub query: crate::datadogV1_20270101::model::EventsPlatformDataProjectionQuery,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "request_type")]
    pub request_type: crate::datadogV1_20270101::model::PointPlotDataProjectionRequestRequestType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl PointPlotDataProjectionRequest {
    pub fn new(projection: crate::datadogV1_20270101::model::PointPlotProjection, query: crate::datadogV1_20270101::model::EventsPlatformDataProjectionQuery, request_type: crate::datadogV1_20270101::model::PointPlotDataProjectionRequestRequestType) -> PointPlotDataProjectionRequest {
        PointPlotDataProjectionRequest {
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



impl<'de> Deserialize<'de> for PointPlotDataProjectionRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PointPlotDataProjectionRequestVisitor;
        impl<'a> Visitor<'a> for PointPlotDataProjectionRequestVisitor {
            type Value = PointPlotDataProjectionRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut limit: Option<f64> = None;
                let mut projection: Option<crate::datadogV1_20270101::model::PointPlotProjection> = None;
                let mut query: Option<crate::datadogV1_20270101::model::EventsPlatformDataProjectionQuery> = None;
                let mut request_type: Option<crate::datadogV1_20270101::model::PointPlotDataProjectionRequestRequestType> = None;
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
                                    crate::datadogV1_20270101::model::PointPlotDataProjectionRequestRequestType::UnparsedObject(_request_type) => {
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

                let content = PointPlotDataProjectionRequest {
                    limit,
                    projection,
                    query,
                    request_type,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(PointPlotDataProjectionRequestVisitor)
    }
}

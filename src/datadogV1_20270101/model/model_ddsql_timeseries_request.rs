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
pub struct DDSQLTimeseriesRequest {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "ddsql_query")]
    pub ddsql_query: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "request_type")]
    pub request_type: crate::datadogV1_20270101::model::DDSQLTimeseriesRequestRequestType,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "response_format")]
    pub response_format: crate::datadogV1_20270101::model::DDSQLTimeseriesRequestResponseFormat,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl DDSQLTimeseriesRequest {
    pub fn new(ddsql_query: String, request_type: crate::datadogV1_20270101::model::DDSQLTimeseriesRequestRequestType, response_format: crate::datadogV1_20270101::model::DDSQLTimeseriesRequestResponseFormat) -> DDSQLTimeseriesRequest {
        DDSQLTimeseriesRequest {
            ddsql_query,
            request_type,
            response_format,
            _unparsed: false,
        }
    }
    
}



impl<'de> Deserialize<'de> for DDSQLTimeseriesRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DDSQLTimeseriesRequestVisitor;
        impl<'a> Visitor<'a> for DDSQLTimeseriesRequestVisitor {
            type Value = DDSQLTimeseriesRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut ddsql_query: Option<String> = None;
                let mut request_type: Option<crate::datadogV1_20270101::model::DDSQLTimeseriesRequestRequestType> = None;
                let mut response_format: Option<crate::datadogV1_20270101::model::DDSQLTimeseriesRequestResponseFormat> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "ddsql_query" => {
                            ddsql_query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "request_type" => {
                            request_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _request_type) = request_type {
                                match _request_type {
                                    crate::datadogV1_20270101::model::DDSQLTimeseriesRequestRequestType::UnparsedObject(_request_type) => {
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
                                    crate::datadogV1_20270101::model::DDSQLTimeseriesRequestResponseFormat::UnparsedObject(_response_format) => {
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
                let ddsql_query = ddsql_query.ok_or_else(|| M::Error::missing_field("ddsql_query"))?;
                let request_type = request_type.ok_or_else(|| M::Error::missing_field("request_type"))?;
                let response_format = response_format.ok_or_else(|| M::Error::missing_field("response_format"))?;

                let content = DDSQLTimeseriesRequest {
                    ddsql_query,
                    request_type,
                    response_format,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DDSQLTimeseriesRequestVisitor)
    }
}

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
pub struct RumStreamRequest {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "columns")]
    pub columns: Vec<crate::datadogV1_20270101::model::RumStreamRequestColumnsItems>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "query")]
    pub query: crate::datadogV1_20270101::model::RumStreamQuery,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "response_format")]
    pub response_format: crate::datadogV1_20270101::model::RumStreamRequestResponseFormat,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl RumStreamRequest {
    pub fn new(columns: Vec<crate::datadogV1_20270101::model::RumStreamRequestColumnsItems>, query: crate::datadogV1_20270101::model::RumStreamQuery, response_format: crate::datadogV1_20270101::model::RumStreamRequestResponseFormat) -> RumStreamRequest {
        RumStreamRequest {
            columns,
            query,
            response_format,
            _unparsed: false,
        }
    }
    
}



impl<'de> Deserialize<'de> for RumStreamRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RumStreamRequestVisitor;
        impl<'a> Visitor<'a> for RumStreamRequestVisitor {
            type Value = RumStreamRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut columns: Option<Vec<crate::datadogV1_20270101::model::RumStreamRequestColumnsItems>> = None;
                let mut query: Option<crate::datadogV1_20270101::model::RumStreamQuery> = None;
                let mut response_format: Option<crate::datadogV1_20270101::model::RumStreamRequestResponseFormat> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "columns" => {
                            columns = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "query" => {
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "response_format" => {
                            response_format = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _response_format) = response_format {
                                match _response_format {
                                    crate::datadogV1_20270101::model::RumStreamRequestResponseFormat::UnparsedObject(_response_format) => {
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
                let columns = columns.ok_or_else(|| M::Error::missing_field("columns"))?;
                let query = query.ok_or_else(|| M::Error::missing_field("query"))?;
                let response_format = response_format.ok_or_else(|| M::Error::missing_field("response_format"))?;

                let content = RumStreamRequest {
                    columns,
                    query,
                    response_format,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RumStreamRequestVisitor)
    }
}

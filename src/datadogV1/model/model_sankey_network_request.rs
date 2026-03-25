// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Sankey widget request for network data source.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SankeyNetworkRequest {
    /// Query configuration for Sankey network widget.
    #[serde(rename = "query")]
    pub query: crate::datadogV1::model::SankeyNetworkQuery,
    /// Type of request for network Sankey widget.
    #[serde(rename = "request_type")]
    pub request_type: crate::datadogV1::model::SankeyNetworkRequestType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SankeyNetworkRequest {
    pub fn new(
        query: crate::datadogV1::model::SankeyNetworkQuery,
        request_type: crate::datadogV1::model::SankeyNetworkRequestType,
    ) -> SankeyNetworkRequest {
        SankeyNetworkRequest {
            query,
            request_type,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for SankeyNetworkRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SankeyNetworkRequestVisitor;
        impl<'a> Visitor<'a> for SankeyNetworkRequestVisitor {
            type Value = SankeyNetworkRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut query: Option<crate::datadogV1::model::SankeyNetworkQuery> = None;
                let mut request_type: Option<crate::datadogV1::model::SankeyNetworkRequestType> =
                    None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "query" => {
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "request_type" => {
                            request_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _request_type) = request_type {
                                match _request_type {
                                    crate::datadogV1::model::SankeyNetworkRequestType::UnparsedObject(_request_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let query = query.ok_or_else(|| M::Error::missing_field("query"))?;
                let request_type =
                    request_type.ok_or_else(|| M::Error::missing_field("request_type"))?;

                let content = SankeyNetworkRequest {
                    query,
                    request_type,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SankeyNetworkRequestVisitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Request that will return nodes and edges to be used by topology map.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TopologyRequest {
    /// Query to service-based topology data sources like the service map or data streams.
    #[serde(rename = "query")]
    pub query: Option<crate::datadogV1::model::TopologyQuery>,
    /// Widget request type.
    #[serde(rename = "request_type")]
    pub request_type: Option<crate::datadogV1::model::TopologyRequestType>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TopologyRequest {
    pub fn new() -> TopologyRequest {
        TopologyRequest {
            query: None,
            request_type: None,
            _unparsed: false,
        }
    }

    pub fn query(mut self, value: crate::datadogV1::model::TopologyQuery) -> Self {
        self.query = Some(value);
        self
    }

    pub fn request_type(mut self, value: crate::datadogV1::model::TopologyRequestType) -> Self {
        self.request_type = Some(value);
        self
    }
}

impl Default for TopologyRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TopologyRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TopologyRequestVisitor;
        impl<'a> Visitor<'a> for TopologyRequestVisitor {
            type Value = TopologyRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut query: Option<crate::datadogV1::model::TopologyQuery> = None;
                let mut request_type: Option<crate::datadogV1::model::TopologyRequestType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "query" => {
                            if v.is_null() {
                                continue;
                            }
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "request_type" => {
                            if v.is_null() {
                                continue;
                            }
                            request_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _request_type) = request_type {
                                match _request_type {
                                    crate::datadogV1::model::TopologyRequestType::UnparsedObject(_request_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }

                let content = TopologyRequest {
                    query,
                    request_type,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TopologyRequestVisitor)
    }
}

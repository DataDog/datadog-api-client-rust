// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Sankey widget with RUM data source.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SankeyRumRequest {
    /// Sankey widget with RUM data source query.
    #[serde(rename = "query")]
    pub query: crate::datadogV1::model::SankeyRumQuery,
    /// Type of the Sankey widget.
    #[serde(rename = "request_type")]
    pub request_type: crate::datadogV1::model::SankeyWidgetDefinitionType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SankeyRumRequest {
    pub fn new(
        query: crate::datadogV1::model::SankeyRumQuery,
        request_type: crate::datadogV1::model::SankeyWidgetDefinitionType,
    ) -> SankeyRumRequest {
        SankeyRumRequest {
            query,
            request_type,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for SankeyRumRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SankeyRumRequestVisitor;
        impl<'a> Visitor<'a> for SankeyRumRequestVisitor {
            type Value = SankeyRumRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut query: Option<crate::datadogV1::model::SankeyRumQuery> = None;
                let mut request_type: Option<crate::datadogV1::model::SankeyWidgetDefinitionType> =
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
                                    crate::datadogV1::model::SankeyWidgetDefinitionType::UnparsedObject(_request_type) => {
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

                let content = SankeyRumRequest {
                    query,
                    request_type,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SankeyRumRequestVisitor)
    }
}

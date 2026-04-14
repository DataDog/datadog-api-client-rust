// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// User journey funnel widget request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductAnalyticsFunnelRequest {
    /// Comparison segments.
    #[serde(rename = "comparison_segments")]
    pub comparison_segments: Option<Vec<String>>,
    /// Comparison time configuration for funnel widgets.
    #[serde(rename = "comparison_time")]
    pub comparison_time: Option<crate::datadogV1::model::FunnelComparisonDuration>,
    /// User journey funnel query definition.
    #[serde(rename = "query")]
    pub query: crate::datadogV1::model::ProductAnalyticsFunnelQuery,
    /// Request type for user journey funnel widget.
    #[serde(rename = "request_type")]
    pub request_type: crate::datadogV1::model::ProductAnalyticsFunnelRequestType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductAnalyticsFunnelRequest {
    pub fn new(
        query: crate::datadogV1::model::ProductAnalyticsFunnelQuery,
        request_type: crate::datadogV1::model::ProductAnalyticsFunnelRequestType,
    ) -> ProductAnalyticsFunnelRequest {
        ProductAnalyticsFunnelRequest {
            comparison_segments: None,
            comparison_time: None,
            query,
            request_type,
            _unparsed: false,
        }
    }

    pub fn comparison_segments(mut self, value: Vec<String>) -> Self {
        self.comparison_segments = Some(value);
        self
    }

    pub fn comparison_time(
        mut self,
        value: crate::datadogV1::model::FunnelComparisonDuration,
    ) -> Self {
        self.comparison_time = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for ProductAnalyticsFunnelRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsFunnelRequestVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsFunnelRequestVisitor {
            type Value = ProductAnalyticsFunnelRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut comparison_segments: Option<Vec<String>> = None;
                let mut comparison_time: Option<crate::datadogV1::model::FunnelComparisonDuration> =
                    None;
                let mut query: Option<crate::datadogV1::model::ProductAnalyticsFunnelQuery> = None;
                let mut request_type: Option<
                    crate::datadogV1::model::ProductAnalyticsFunnelRequestType,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "comparison_segments" => {
                            if v.is_null() {
                                continue;
                            }
                            comparison_segments =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "comparison_time" => {
                            if v.is_null() {
                                continue;
                            }
                            comparison_time =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query" => {
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "request_type" => {
                            request_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _request_type) = request_type {
                                match _request_type {
                                    crate::datadogV1::model::ProductAnalyticsFunnelRequestType::UnparsedObject(_request_type) => {
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

                let content = ProductAnalyticsFunnelRequest {
                    comparison_segments,
                    comparison_time,
                    query,
                    request_type,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductAnalyticsFunnelRequestVisitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The response object for the pipeline events aggregate API endpoint.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CIAppPipelinesAnalyticsAggregateResponse {
    /// The query results.
    #[serde(rename = "data")]
    pub data: Option<crate::datadogV2::model::CIAppPipelinesAggregationBucketsResponse>,
    /// Links attributes.
    #[serde(rename = "links")]
    pub links: Option<crate::datadogV2::model::CIAppResponseLinks>,
    /// The metadata associated with a request.
    #[serde(rename = "meta")]
    pub meta: Option<crate::datadogV2::model::CIAppResponseMetadata>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CIAppPipelinesAnalyticsAggregateResponse {
    pub fn new() -> CIAppPipelinesAnalyticsAggregateResponse {
        CIAppPipelinesAnalyticsAggregateResponse {
            data: None,
            links: None,
            meta: None,
            _unparsed: false,
        }
    }

    pub fn data(
        &mut self,
        value: crate::datadogV2::model::CIAppPipelinesAggregationBucketsResponse,
    ) -> &mut Self {
        self.data = Some(value);
        self
    }

    pub fn links(&mut self, value: crate::datadogV2::model::CIAppResponseLinks) -> &mut Self {
        self.links = Some(value);
        self
    }

    pub fn meta(&mut self, value: crate::datadogV2::model::CIAppResponseMetadata) -> &mut Self {
        self.meta = Some(value);
        self
    }
}

impl Default for CIAppPipelinesAnalyticsAggregateResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CIAppPipelinesAnalyticsAggregateResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CIAppPipelinesAnalyticsAggregateResponseVisitor;
        impl<'a> Visitor<'a> for CIAppPipelinesAnalyticsAggregateResponseVisitor {
            type Value = CIAppPipelinesAnalyticsAggregateResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data: Option<
                    crate::datadogV2::model::CIAppPipelinesAggregationBucketsResponse,
                > = None;
                let mut links: Option<crate::datadogV2::model::CIAppResponseLinks> = None;
                let mut meta: Option<crate::datadogV2::model::CIAppResponseMetadata> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data" => {
                            if v.is_null() {
                                continue;
                            }
                            data = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "links" => {
                            if v.is_null() {
                                continue;
                            }
                            links = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "meta" => {
                            if v.is_null() {
                                continue;
                            }
                            meta = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = CIAppPipelinesAnalyticsAggregateResponse {
                    data,
                    links,
                    meta,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CIAppPipelinesAnalyticsAggregateResponseVisitor)
    }
}

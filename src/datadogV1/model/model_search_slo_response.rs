// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A search SLO response containing results from the search query.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SearchSLOResponse {
    /// Data from search SLO response.
    #[serde(rename = "data")]
    pub data: Option<crate::datadogV1::model::SearchSLOResponseData>,
    /// Pagination links.
    #[serde(rename = "links")]
    pub links: Option<crate::datadogV1::model::SearchSLOResponseLinks>,
    /// Searches metadata returned by the API.
    #[serde(rename = "meta")]
    pub meta: Option<crate::datadogV1::model::SearchSLOResponseMeta>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SearchSLOResponse {
    pub fn new() -> SearchSLOResponse {
        SearchSLOResponse {
            data: None,
            links: None,
            meta: None,
            _unparsed: false,
        }
    }

    pub fn data(&mut self, value: crate::datadogV1::model::SearchSLOResponseData) -> &mut Self {
        self.data = Some(value);
        self
    }

    pub fn links(&mut self, value: crate::datadogV1::model::SearchSLOResponseLinks) -> &mut Self {
        self.links = Some(value);
        self
    }

    pub fn meta(&mut self, value: crate::datadogV1::model::SearchSLOResponseMeta) -> &mut Self {
        self.meta = Some(value);
        self
    }
}

impl Default for SearchSLOResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SearchSLOResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SearchSLOResponseVisitor;
        impl<'a> Visitor<'a> for SearchSLOResponseVisitor {
            type Value = SearchSLOResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data: Option<crate::datadogV1::model::SearchSLOResponseData> = None;
                let mut links: Option<crate::datadogV1::model::SearchSLOResponseLinks> = None;
                let mut meta: Option<crate::datadogV1::model::SearchSLOResponseMeta> = None;
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

                let content = SearchSLOResponse {
                    data,
                    links,
                    meta,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SearchSLOResponseVisitor)
    }
}

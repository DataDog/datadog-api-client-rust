// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Additional information related to api keys response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct APIKeysResponseMeta {
    /// Max allowed number of API keys.
    #[serde(rename = "max_allowed")]
    pub max_allowed: Option<i64>,
    /// Additional information related to the API keys response.
    #[serde(rename = "page")]
    pub page: Option<crate::datadogV2::model::APIKeysResponseMetaPage>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl APIKeysResponseMeta {
    pub fn new() -> APIKeysResponseMeta {
        APIKeysResponseMeta {
            max_allowed: None,
            page: None,
            _unparsed: false,
        }
    }

    pub fn max_allowed(mut self, value: i64) -> Self {
        self.max_allowed = Some(value);
        self
    }

    pub fn page(mut self, value: crate::datadogV2::model::APIKeysResponseMetaPage) -> Self {
        self.page = Some(value);
        self
    }
}

impl Default for APIKeysResponseMeta {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for APIKeysResponseMeta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct APIKeysResponseMetaVisitor;
        impl<'a> Visitor<'a> for APIKeysResponseMetaVisitor {
            type Value = APIKeysResponseMeta;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut max_allowed: Option<i64> = None;
                let mut page: Option<crate::datadogV2::model::APIKeysResponseMetaPage> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "max_allowed" => {
                            if v.is_null() {
                                continue;
                            }
                            max_allowed =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "page" => {
                            if v.is_null() {
                                continue;
                            }
                            page = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = APIKeysResponseMeta {
                    max_allowed,
                    page,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(APIKeysResponseMetaVisitor)
    }
}

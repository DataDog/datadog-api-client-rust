// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Response for a list of API keys.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct APIKeysResponse {
    /// Array of API keys.
    #[serde(rename = "data")]
    pub data: Option<Vec<crate::datadogV2::model::PartialAPIKey>>,
    /// Array of objects related to the API key.
    #[serde(rename = "included")]
    pub included: Option<Vec<crate::datadogV2::model::APIKeyResponseIncludedItem>>,
    /// Additional information related to api keys response.
    #[serde(rename = "meta")]
    pub meta: Option<crate::datadogV2::model::APIKeysResponseMeta>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl APIKeysResponse {
    pub fn new() -> APIKeysResponse {
        APIKeysResponse {
            data: None,
            included: None,
            meta: None,
            _unparsed: false,
        }
    }

    pub fn data(mut self, value: Vec<crate::datadogV2::model::PartialAPIKey>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn included(
        mut self,
        value: Vec<crate::datadogV2::model::APIKeyResponseIncludedItem>,
    ) -> Self {
        self.included = Some(value);
        self
    }

    pub fn meta(mut self, value: crate::datadogV2::model::APIKeysResponseMeta) -> Self {
        self.meta = Some(value);
        self
    }
}

impl Default for APIKeysResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for APIKeysResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct APIKeysResponseVisitor;
        impl<'a> Visitor<'a> for APIKeysResponseVisitor {
            type Value = APIKeysResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data: Option<Vec<crate::datadogV2::model::PartialAPIKey>> = None;
                let mut included: Option<Vec<crate::datadogV2::model::APIKeyResponseIncludedItem>> =
                    None;
                let mut meta: Option<crate::datadogV2::model::APIKeysResponseMeta> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data" => {
                            if v.is_null() {
                                continue;
                            }
                            data = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "included" => {
                            if v.is_null() {
                                continue;
                            }
                            included = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = APIKeysResponse {
                    data,
                    included,
                    meta,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(APIKeysResponseVisitor)
    }
}

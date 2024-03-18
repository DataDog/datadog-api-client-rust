// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// List of API and application keys available for a given organization.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ApiKeyListResponse {
    /// Array of API keys.
    #[serde(rename = "api_keys")]
    pub api_keys: Option<Vec<crate::datadogV1::model::ApiKey>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ApiKeyListResponse {
    pub fn new() -> ApiKeyListResponse {
        ApiKeyListResponse {
            api_keys: None,
            _unparsed: false,
        }
    }

    pub fn api_keys(mut self, value: Vec<crate::datadogV1::model::ApiKey>) -> Self {
        self.api_keys = Some(value);
        self
    }
}

impl Default for ApiKeyListResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ApiKeyListResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ApiKeyListResponseVisitor;
        impl<'a> Visitor<'a> for ApiKeyListResponseVisitor {
            type Value = ApiKeyListResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut api_keys: Option<Vec<crate::datadogV1::model::ApiKey>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "api_keys" => {
                            if v.is_null() {
                                continue;
                            }
                            api_keys = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = ApiKeyListResponse {
                    api_keys,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ApiKeyListResponseVisitor)
    }
}

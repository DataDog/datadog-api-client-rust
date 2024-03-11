// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// An API key with its associated metadata.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ApiKeyResponse {
    /// Datadog API key.
    #[serde(rename = "api_key")]
    pub api_key: Option<crate::datadogV1::model::ApiKey>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ApiKeyResponse {
    pub fn new() -> ApiKeyResponse {
        ApiKeyResponse {
            api_key: None,
            _unparsed: false,
        }
    }

    pub fn api_key(&mut self, value: crate::datadogV1::model::ApiKey) -> &mut Self {
        self.api_key = Some(value);
        self
    }
}

impl Default for ApiKeyResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ApiKeyResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ApiKeyResponseVisitor;
        impl<'a> Visitor<'a> for ApiKeyResponseVisitor {
            type Value = ApiKeyResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut api_key: Option<crate::datadogV1::model::ApiKey> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "api_key" => {
                            if v.is_null() {
                                continue;
                            }
                            api_key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = ApiKeyResponse { api_key, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ApiKeyResponseVisitor)
    }
}

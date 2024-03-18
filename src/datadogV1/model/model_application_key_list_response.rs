// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// An application key response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ApplicationKeyListResponse {
    /// Array of application keys.
    #[serde(rename = "application_keys")]
    pub application_keys: Option<Vec<crate::datadogV1::model::ApplicationKey>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ApplicationKeyListResponse {
    pub fn new() -> ApplicationKeyListResponse {
        ApplicationKeyListResponse {
            application_keys: None,
            _unparsed: false,
        }
    }

    pub fn application_keys(mut self, value: Vec<crate::datadogV1::model::ApplicationKey>) -> Self {
        self.application_keys = Some(value);
        self
    }
}

impl Default for ApplicationKeyListResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ApplicationKeyListResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ApplicationKeyListResponseVisitor;
        impl<'a> Visitor<'a> for ApplicationKeyListResponseVisitor {
            type Value = ApplicationKeyListResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut application_keys: Option<Vec<crate::datadogV1::model::ApplicationKey>> =
                    None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "application_keys" => {
                            if v.is_null() {
                                continue;
                            }
                            application_keys =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = ApplicationKeyListResponse {
                    application_keys,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ApplicationKeyListResponseVisitor)
    }
}

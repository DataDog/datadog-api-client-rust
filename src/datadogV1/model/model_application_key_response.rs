// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// An application key response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ApplicationKeyResponse {
    /// An application key with its associated metadata.
    #[serde(rename = "application_key")]
    pub application_key: Option<crate::datadogV1::model::ApplicationKey>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ApplicationKeyResponse {
    pub fn new() -> ApplicationKeyResponse {
        ApplicationKeyResponse {
            application_key: None,
            _unparsed: false,
        }
    }

    pub fn application_key(mut self, value: crate::datadogV1::model::ApplicationKey) -> Self {
        self.application_key = Some(value);
        self
    }
}

impl Default for ApplicationKeyResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ApplicationKeyResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ApplicationKeyResponseVisitor;
        impl<'a> Visitor<'a> for ApplicationKeyResponseVisitor {
            type Value = ApplicationKeyResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut application_key: Option<crate::datadogV1::model::ApplicationKey> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "application_key" => {
                            if v.is_null() {
                                continue;
                            }
                            application_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = ApplicationKeyResponse {
                    application_key,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ApplicationKeyResponseVisitor)
    }
}

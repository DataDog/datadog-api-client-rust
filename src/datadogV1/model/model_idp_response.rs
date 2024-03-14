// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The IdP response object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IdpResponse {
    /// Identity provider response.
    #[serde(rename = "message")]
    pub message: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IdpResponse {
    pub fn new(message: String) -> IdpResponse {
        IdpResponse {
            message,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for IdpResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IdpResponseVisitor;
        impl<'a> Visitor<'a> for IdpResponseVisitor {
            type Value = IdpResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut message: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "message" => {
                            message = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let message = message.ok_or_else(|| M::Error::missing_field("message"))?;

                let content = IdpResponse { message, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IdpResponseVisitor)
    }
}

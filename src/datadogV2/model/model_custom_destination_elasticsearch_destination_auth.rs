// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Basic access authentication.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CustomDestinationElasticsearchDestinationAuth {
    /// The password of the authentication. This field is not returned by the API.
    #[serde(rename = "password")]
    pub password: String,
    /// The username of the authentication. This field is not returned by the API.
    #[serde(rename = "username")]
    pub username: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CustomDestinationElasticsearchDestinationAuth {
    pub fn new(
        password: String,
        username: String,
    ) -> CustomDestinationElasticsearchDestinationAuth {
        CustomDestinationElasticsearchDestinationAuth {
            password,
            username,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for CustomDestinationElasticsearchDestinationAuth {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CustomDestinationElasticsearchDestinationAuthVisitor;
        impl<'a> Visitor<'a> for CustomDestinationElasticsearchDestinationAuthVisitor {
            type Value = CustomDestinationElasticsearchDestinationAuth;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut password: Option<String> = None;
                let mut username: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "password" => {
                            password = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "username" => {
                            username = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let password = password.ok_or_else(|| M::Error::missing_field("password"))?;
                let username = username.ok_or_else(|| M::Error::missing_field("username"))?;

                let content = CustomDestinationElasticsearchDestinationAuth {
                    password,
                    username,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CustomDestinationElasticsearchDestinationAuthVisitor)
    }
}

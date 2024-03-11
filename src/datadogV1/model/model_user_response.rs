// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A Datadog User.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UserResponse {
    /// Create, edit, and disable users.
    #[serde(rename = "user")]
    pub user: Option<crate::datadogV1::model::User>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UserResponse {
    pub fn new() -> UserResponse {
        UserResponse {
            user: None,
            _unparsed: false,
        }
    }

    pub fn user(&mut self, value: crate::datadogV1::model::User) -> &mut Self {
        self.user = Some(value);
        self
    }
}

impl Default for UserResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for UserResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UserResponseVisitor;
        impl<'a> Visitor<'a> for UserResponseVisitor {
            type Value = UserResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut user: Option<crate::datadogV1::model::User> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "user" => {
                            if v.is_null() {
                                continue;
                            }
                            user = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = UserResponse { user, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UserResponseVisitor)
    }
}

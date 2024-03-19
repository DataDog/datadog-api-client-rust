// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Array of Datadog users for a given organization.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UserListResponse {
    /// Array of users.
    #[serde(rename = "users")]
    pub users: Option<Vec<crate::datadogV1::model::User>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UserListResponse {
    pub fn new() -> UserListResponse {
        UserListResponse {
            users: None,
            _unparsed: false,
        }
    }

    pub fn users(mut self, value: Vec<crate::datadogV1::model::User>) -> Self {
        self.users = Some(value);
        self
    }
}

impl Default for UserListResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for UserListResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UserListResponseVisitor;
        impl<'a> Visitor<'a> for UserListResponseVisitor {
            type Value = UserListResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut users: Option<Vec<crate::datadogV1::model::User>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "users" => {
                            if v.is_null() {
                                continue;
                            }
                            users = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = UserListResponse { users, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UserListResponseVisitor)
    }
}

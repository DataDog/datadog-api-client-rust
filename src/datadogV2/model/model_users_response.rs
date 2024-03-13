// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Response containing information about multiple users.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UsersResponse {
    /// Array of returned users.
    #[serde(rename = "data")]
    pub data: Option<Vec<crate::datadogV2::model::User>>,
    /// Array of objects related to the users.
    #[serde(rename = "included")]
    pub included: Option<Vec<crate::datadogV2::model::UserResponseIncludedItem>>,
    /// Object describing meta attributes of response.
    #[serde(rename = "meta")]
    pub meta: Option<crate::datadogV2::model::ResponseMetaAttributes>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UsersResponse {
    pub fn new() -> UsersResponse {
        UsersResponse {
            data: None,
            included: None,
            meta: None,
            _unparsed: false,
        }
    }

    pub fn data(mut self, value: Vec<crate::datadogV2::model::User>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn included(
        mut self,
        value: Vec<crate::datadogV2::model::UserResponseIncludedItem>,
    ) -> Self {
        self.included = Some(value);
        self
    }

    pub fn meta(mut self, value: crate::datadogV2::model::ResponseMetaAttributes) -> Self {
        self.meta = Some(value);
        self
    }
}

impl Default for UsersResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for UsersResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UsersResponseVisitor;
        impl<'a> Visitor<'a> for UsersResponseVisitor {
            type Value = UsersResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data: Option<Vec<crate::datadogV2::model::User>> = None;
                let mut included: Option<Vec<crate::datadogV2::model::UserResponseIncludedItem>> =
                    None;
                let mut meta: Option<crate::datadogV2::model::ResponseMetaAttributes> = None;
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

                let content = UsersResponse {
                    data,
                    included,
                    meta,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UsersResponseVisitor)
    }
}

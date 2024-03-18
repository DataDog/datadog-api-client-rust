// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object to handle basic authentication when performing the test.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsBasicAuthWeb {
    /// Password to use for the basic authentication.
    #[serde(rename = "password")]
    pub password: String,
    /// The type of basic authentication to use when performing the test.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV1::model::SyntheticsBasicAuthWebType>,
    /// Username to use for the basic authentication.
    #[serde(rename = "username")]
    pub username: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsBasicAuthWeb {
    pub fn new(password: String, username: String) -> SyntheticsBasicAuthWeb {
        SyntheticsBasicAuthWeb {
            password,
            type_: None,
            username,
            _unparsed: false,
        }
    }

    pub fn type_(mut self, value: crate::datadogV1::model::SyntheticsBasicAuthWebType) -> Self {
        self.type_ = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for SyntheticsBasicAuthWeb {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsBasicAuthWebVisitor;
        impl<'a> Visitor<'a> for SyntheticsBasicAuthWebVisitor {
            type Value = SyntheticsBasicAuthWeb;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut password: Option<String> = None;
                let mut type_: Option<crate::datadogV1::model::SyntheticsBasicAuthWebType> = None;
                let mut username: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "password" => {
                            password = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::SyntheticsBasicAuthWebType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "username" => {
                            username = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let password = password.ok_or_else(|| M::Error::missing_field("password"))?;
                let username = username.ok_or_else(|| M::Error::missing_field("username"))?;

                let content = SyntheticsBasicAuthWeb {
                    password,
                    type_,
                    username,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsBasicAuthWebVisitor)
    }
}

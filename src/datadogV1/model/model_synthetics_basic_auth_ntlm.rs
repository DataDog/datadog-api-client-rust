// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object to handle `NTLM` authentication when performing the test.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsBasicAuthNTLM {
    /// Domain for the authentication to use when performing the test.
    #[serde(rename = "domain")]
    pub domain: Option<String>,
    /// Password for the authentication to use when performing the test.
    #[serde(rename = "password")]
    pub password: Option<String>,
    /// The type of authentication to use when performing the test.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::SyntheticsBasicAuthNTLMType,
    /// Username for the authentication to use when performing the test.
    #[serde(rename = "username")]
    pub username: Option<String>,
    /// Workstation for the authentication to use when performing the test.
    #[serde(rename = "workstation")]
    pub workstation: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsBasicAuthNTLM {
    pub fn new(
        type_: crate::datadogV1::model::SyntheticsBasicAuthNTLMType,
    ) -> SyntheticsBasicAuthNTLM {
        SyntheticsBasicAuthNTLM {
            domain: None,
            password: None,
            type_,
            username: None,
            workstation: None,
            _unparsed: false,
        }
    }

    pub fn domain(mut self, value: String) -> Self {
        self.domain = Some(value);
        self
    }

    pub fn password(mut self, value: String) -> Self {
        self.password = Some(value);
        self
    }

    pub fn username(mut self, value: String) -> Self {
        self.username = Some(value);
        self
    }

    pub fn workstation(mut self, value: String) -> Self {
        self.workstation = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for SyntheticsBasicAuthNTLM {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsBasicAuthNTLMVisitor;
        impl<'a> Visitor<'a> for SyntheticsBasicAuthNTLMVisitor {
            type Value = SyntheticsBasicAuthNTLM;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut domain: Option<String> = None;
                let mut password: Option<String> = None;
                let mut type_: Option<crate::datadogV1::model::SyntheticsBasicAuthNTLMType> = None;
                let mut username: Option<String> = None;
                let mut workstation: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "domain" => {
                            if v.is_null() {
                                continue;
                            }
                            domain = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "password" => {
                            if v.is_null() {
                                continue;
                            }
                            password = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::SyntheticsBasicAuthNTLMType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "username" => {
                            if v.is_null() {
                                continue;
                            }
                            username = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "workstation" => {
                            if v.is_null() {
                                continue;
                            }
                            workstation =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = SyntheticsBasicAuthNTLM {
                    domain,
                    password,
                    type_,
                    username,
                    workstation,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsBasicAuthNTLMVisitor)
    }
}

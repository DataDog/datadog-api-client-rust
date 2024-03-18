// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// List of AWS accounts to delete.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AWSAccountDeleteRequest {
    /// Your AWS access key ID. Only required if your AWS account is a GovCloud or China account.
    #[serde(rename = "access_key_id")]
    pub access_key_id: Option<String>,
    /// Your AWS Account ID without dashes.
    #[serde(rename = "account_id")]
    pub account_id: Option<String>,
    /// Your Datadog role delegation name.
    #[serde(rename = "role_name")]
    pub role_name: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AWSAccountDeleteRequest {
    pub fn new() -> AWSAccountDeleteRequest {
        AWSAccountDeleteRequest {
            access_key_id: None,
            account_id: None,
            role_name: None,
            _unparsed: false,
        }
    }

    pub fn access_key_id(mut self, value: String) -> Self {
        self.access_key_id = Some(value);
        self
    }

    pub fn account_id(mut self, value: String) -> Self {
        self.account_id = Some(value);
        self
    }

    pub fn role_name(mut self, value: String) -> Self {
        self.role_name = Some(value);
        self
    }
}

impl Default for AWSAccountDeleteRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AWSAccountDeleteRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AWSAccountDeleteRequestVisitor;
        impl<'a> Visitor<'a> for AWSAccountDeleteRequestVisitor {
            type Value = AWSAccountDeleteRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut access_key_id: Option<String> = None;
                let mut account_id: Option<String> = None;
                let mut role_name: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "access_key_id" => {
                            if v.is_null() {
                                continue;
                            }
                            access_key_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "account_id" => {
                            if v.is_null() {
                                continue;
                            }
                            account_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "role_name" => {
                            if v.is_null() {
                                continue;
                            }
                            role_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = AWSAccountDeleteRequest {
                    access_key_id,
                    account_id,
                    role_name,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AWSAccountDeleteRequestVisitor)
    }
}

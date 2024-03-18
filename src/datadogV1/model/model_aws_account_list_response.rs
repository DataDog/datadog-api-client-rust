// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// List of enabled AWS accounts.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AWSAccountListResponse {
    /// List of enabled AWS accounts.
    #[serde(rename = "accounts")]
    pub accounts: Option<Vec<crate::datadogV1::model::AWSAccount>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AWSAccountListResponse {
    pub fn new() -> AWSAccountListResponse {
        AWSAccountListResponse {
            accounts: None,
            _unparsed: false,
        }
    }

    pub fn accounts(mut self, value: Vec<crate::datadogV1::model::AWSAccount>) -> Self {
        self.accounts = Some(value);
        self
    }
}

impl Default for AWSAccountListResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AWSAccountListResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AWSAccountListResponseVisitor;
        impl<'a> Visitor<'a> for AWSAccountListResponseVisitor {
            type Value = AWSAccountListResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut accounts: Option<Vec<crate::datadogV1::model::AWSAccount>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "accounts" => {
                            if v.is_null() {
                                continue;
                            }
                            accounts = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = AWSAccountListResponse {
                    accounts,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AWSAccountListResponseVisitor)
    }
}

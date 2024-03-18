// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// An object describing the EventBridge configuration for multiple accounts.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AWSEventBridgeListResponse {
    /// List of accounts with their event sources.
    #[serde(rename = "accounts")]
    pub accounts: Option<Vec<crate::datadogV1::model::AWSEventBridgeAccountConfiguration>>,
    /// True if the EventBridge sub-integration is enabled for your organization.
    #[serde(rename = "isInstalled")]
    pub is_installed: Option<bool>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AWSEventBridgeListResponse {
    pub fn new() -> AWSEventBridgeListResponse {
        AWSEventBridgeListResponse {
            accounts: None,
            is_installed: None,
            _unparsed: false,
        }
    }

    pub fn accounts(
        mut self,
        value: Vec<crate::datadogV1::model::AWSEventBridgeAccountConfiguration>,
    ) -> Self {
        self.accounts = Some(value);
        self
    }

    pub fn is_installed(mut self, value: bool) -> Self {
        self.is_installed = Some(value);
        self
    }
}

impl Default for AWSEventBridgeListResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AWSEventBridgeListResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AWSEventBridgeListResponseVisitor;
        impl<'a> Visitor<'a> for AWSEventBridgeListResponseVisitor {
            type Value = AWSEventBridgeListResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut accounts: Option<
                    Vec<crate::datadogV1::model::AWSEventBridgeAccountConfiguration>,
                > = None;
                let mut is_installed: Option<bool> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "accounts" => {
                            if v.is_null() {
                                continue;
                            }
                            accounts = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "isInstalled" => {
                            if v.is_null() {
                                continue;
                            }
                            is_installed =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = AWSEventBridgeListResponse {
                    accounts,
                    is_installed,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AWSEventBridgeListResponseVisitor)
    }
}

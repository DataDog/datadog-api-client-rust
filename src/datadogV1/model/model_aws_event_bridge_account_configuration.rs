// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The EventBridge configuration for one AWS account.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AWSEventBridgeAccountConfiguration {
    /// Your AWS Account ID without dashes.
    #[serde(rename = "accountId")]
    pub account_id: Option<String>,
    /// Array of AWS event sources associated with this account.
    #[serde(rename = "eventHubs")]
    pub event_hubs: Option<Vec<crate::datadogV1::model::AWSEventBridgeSource>>,
    /// Array of tags (in the form `key:value`) which are added to all hosts
    /// and metrics reporting through the main AWS integration.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AWSEventBridgeAccountConfiguration {
    pub fn new() -> AWSEventBridgeAccountConfiguration {
        AWSEventBridgeAccountConfiguration {
            account_id: None,
            event_hubs: None,
            tags: None,
            _unparsed: false,
        }
    }

    pub fn account_id(mut self, value: String) -> Self {
        self.account_id = Some(value);
        self
    }

    pub fn event_hubs(mut self, value: Vec<crate::datadogV1::model::AWSEventBridgeSource>) -> Self {
        self.event_hubs = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }
}

impl Default for AWSEventBridgeAccountConfiguration {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AWSEventBridgeAccountConfiguration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AWSEventBridgeAccountConfigurationVisitor;
        impl<'a> Visitor<'a> for AWSEventBridgeAccountConfigurationVisitor {
            type Value = AWSEventBridgeAccountConfiguration;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut account_id: Option<String> = None;
                let mut event_hubs: Option<Vec<crate::datadogV1::model::AWSEventBridgeSource>> =
                    None;
                let mut tags: Option<Vec<String>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "accountId" => {
                            if v.is_null() {
                                continue;
                            }
                            account_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "eventHubs" => {
                            if v.is_null() {
                                continue;
                            }
                            event_hubs = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = AWSEventBridgeAccountConfiguration {
                    account_id,
                    event_hubs,
                    tags,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AWSEventBridgeAccountConfigurationVisitor)
    }
}

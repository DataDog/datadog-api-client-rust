// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// An object used to delete an EventBridge source.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AWSEventBridgeDeleteRequest {
    /// Your AWS Account ID without dashes.
    #[serde(rename = "account_id")]
    pub account_id: Option<String>,
    /// The event source name.
    #[serde(rename = "event_generator_name")]
    pub event_generator_name: Option<String>,
    /// The event source's [AWS region](<https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints>).
    #[serde(rename = "region")]
    pub region: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AWSEventBridgeDeleteRequest {
    pub fn new() -> AWSEventBridgeDeleteRequest {
        AWSEventBridgeDeleteRequest {
            account_id: None,
            event_generator_name: None,
            region: None,
            _unparsed: false,
        }
    }

    pub fn account_id(mut self, value: String) -> Self {
        self.account_id = Some(value);
        self
    }

    pub fn event_generator_name(mut self, value: String) -> Self {
        self.event_generator_name = Some(value);
        self
    }

    pub fn region(mut self, value: String) -> Self {
        self.region = Some(value);
        self
    }
}

impl Default for AWSEventBridgeDeleteRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AWSEventBridgeDeleteRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AWSEventBridgeDeleteRequestVisitor;
        impl<'a> Visitor<'a> for AWSEventBridgeDeleteRequestVisitor {
            type Value = AWSEventBridgeDeleteRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut account_id: Option<String> = None;
                let mut event_generator_name: Option<String> = None;
                let mut region: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "account_id" => {
                            if v.is_null() {
                                continue;
                            }
                            account_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "event_generator_name" => {
                            if v.is_null() {
                                continue;
                            }
                            event_generator_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "region" => {
                            if v.is_null() {
                                continue;
                            }
                            region = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = AWSEventBridgeDeleteRequest {
                    account_id,
                    event_generator_name,
                    region,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AWSEventBridgeDeleteRequestVisitor)
    }
}

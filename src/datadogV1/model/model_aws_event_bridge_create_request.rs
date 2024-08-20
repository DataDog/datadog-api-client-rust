// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// An object used to create an EventBridge source.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AWSEventBridgeCreateRequest {
    /// Your AWS Account ID without dashes.
    #[serde(rename = "account_id")]
    pub account_id: Option<String>,
    /// True if Datadog should create the event bus in addition to the event
    /// source. Requires the `events:CreateEventBus` permission.
    #[serde(rename = "create_event_bus")]
    pub create_event_bus: Option<bool>,
    /// The given part of the event source name, which is then combined with an
    /// assigned suffix to form the full name.
    #[serde(rename = "event_generator_name")]
    pub event_generator_name: Option<String>,
    /// The event source's [AWS region](<https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints>).
    #[serde(rename = "region")]
    pub region: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AWSEventBridgeCreateRequest {
    pub fn new() -> AWSEventBridgeCreateRequest {
        AWSEventBridgeCreateRequest {
            account_id: None,
            create_event_bus: None,
            event_generator_name: None,
            region: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn account_id(mut self, value: String) -> Self {
        self.account_id = Some(value);
        self
    }

    pub fn create_event_bus(mut self, value: bool) -> Self {
        self.create_event_bus = Some(value);
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

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl Default for AWSEventBridgeCreateRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AWSEventBridgeCreateRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AWSEventBridgeCreateRequestVisitor;
        impl<'a> Visitor<'a> for AWSEventBridgeCreateRequestVisitor {
            type Value = AWSEventBridgeCreateRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut account_id: Option<String> = None;
                let mut create_event_bus: Option<bool> = None;
                let mut event_generator_name: Option<String> = None;
                let mut region: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "account_id" => {
                            if v.is_null() {
                                continue;
                            }
                            account_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "create_event_bus" => {
                            if v.is_null() {
                                continue;
                            }
                            create_event_bus =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = AWSEventBridgeCreateRequest {
                    account_id,
                    create_event_bus,
                    event_generator_name,
                    region,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AWSEventBridgeCreateRequestVisitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A created EventBridge source.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AWSEventBridgeCreateResponse {
    /// The event source name.
    #[serde(rename = "event_source_name")]
    pub event_source_name: Option<String>,
    /// True if the event bus was created in addition to the source.
    #[serde(rename = "has_bus")]
    pub has_bus: Option<bool>,
    /// The event source's [AWS region](<https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints>).
    #[serde(rename = "region")]
    pub region: Option<String>,
    /// The event source status "created".
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV1::model::AWSEventBridgeCreateStatus>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AWSEventBridgeCreateResponse {
    pub fn new() -> AWSEventBridgeCreateResponse {
        AWSEventBridgeCreateResponse {
            event_source_name: None,
            has_bus: None,
            region: None,
            status: None,
            _unparsed: false,
        }
    }

    pub fn event_source_name(&mut self, value: String) -> &mut Self {
        self.event_source_name = Some(value);
        self
    }

    pub fn has_bus(&mut self, value: bool) -> &mut Self {
        self.has_bus = Some(value);
        self
    }

    pub fn region(&mut self, value: String) -> &mut Self {
        self.region = Some(value);
        self
    }

    pub fn status(
        &mut self,
        value: crate::datadogV1::model::AWSEventBridgeCreateStatus,
    ) -> &mut Self {
        self.status = Some(value);
        self
    }
}

impl Default for AWSEventBridgeCreateResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AWSEventBridgeCreateResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AWSEventBridgeCreateResponseVisitor;
        impl<'a> Visitor<'a> for AWSEventBridgeCreateResponseVisitor {
            type Value = AWSEventBridgeCreateResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut event_source_name: Option<String> = None;
                let mut has_bus: Option<bool> = None;
                let mut region: Option<String> = None;
                let mut status: Option<crate::datadogV1::model::AWSEventBridgeCreateStatus> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "event_source_name" => {
                            if v.is_null() {
                                continue;
                            }
                            event_source_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "has_bus" => {
                            if v.is_null() {
                                continue;
                            }
                            has_bus = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "region" => {
                            if v.is_null() {
                                continue;
                            }
                            region = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV1::model::AWSEventBridgeCreateStatus::UnparsedObject(_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }

                let content = AWSEventBridgeCreateResponse {
                    event_source_name,
                    has_bus,
                    region,
                    status,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AWSEventBridgeCreateResponseVisitor)
    }
}

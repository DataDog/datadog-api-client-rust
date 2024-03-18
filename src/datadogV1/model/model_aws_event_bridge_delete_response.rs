// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// An indicator of the successful deletion of an EventBridge source.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AWSEventBridgeDeleteResponse {
    /// The event source status "empty".
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV1::model::AWSEventBridgeDeleteStatus>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AWSEventBridgeDeleteResponse {
    pub fn new() -> AWSEventBridgeDeleteResponse {
        AWSEventBridgeDeleteResponse {
            status: None,
            _unparsed: false,
        }
    }

    pub fn status(mut self, value: crate::datadogV1::model::AWSEventBridgeDeleteStatus) -> Self {
        self.status = Some(value);
        self
    }
}

impl Default for AWSEventBridgeDeleteResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AWSEventBridgeDeleteResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AWSEventBridgeDeleteResponseVisitor;
        impl<'a> Visitor<'a> for AWSEventBridgeDeleteResponseVisitor {
            type Value = AWSEventBridgeDeleteResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut status: Option<crate::datadogV1::model::AWSEventBridgeDeleteStatus> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV1::model::AWSEventBridgeDeleteStatus::UnparsedObject(_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }

                let content = AWSEventBridgeDeleteResponse { status, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AWSEventBridgeDeleteResponseVisitor)
    }
}

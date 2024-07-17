// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Count of the device interfaces by status
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DeviceAttributesInterfaceStatuses {
    /// The number of interfaces that are down
    #[serde(rename = "down")]
    pub down: Option<i64>,
    /// The number of interfaces that are off
    #[serde(rename = "off")]
    pub off: Option<i64>,
    /// The number of interfaces that are up
    #[serde(rename = "up")]
    pub up: Option<i64>,
    /// The number of interfaces that are in a warning state
    #[serde(rename = "warning")]
    pub warning: Option<i64>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DeviceAttributesInterfaceStatuses {
    pub fn new() -> DeviceAttributesInterfaceStatuses {
        DeviceAttributesInterfaceStatuses {
            down: None,
            off: None,
            up: None,
            warning: None,
            _unparsed: false,
        }
    }

    pub fn down(mut self, value: i64) -> Self {
        self.down = Some(value);
        self
    }

    pub fn off(mut self, value: i64) -> Self {
        self.off = Some(value);
        self
    }

    pub fn up(mut self, value: i64) -> Self {
        self.up = Some(value);
        self
    }

    pub fn warning(mut self, value: i64) -> Self {
        self.warning = Some(value);
        self
    }
}

impl Default for DeviceAttributesInterfaceStatuses {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for DeviceAttributesInterfaceStatuses {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DeviceAttributesInterfaceStatusesVisitor;
        impl<'a> Visitor<'a> for DeviceAttributesInterfaceStatusesVisitor {
            type Value = DeviceAttributesInterfaceStatuses;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut down: Option<i64> = None;
                let mut off: Option<i64> = None;
                let mut up: Option<i64> = None;
                let mut warning: Option<i64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "down" => {
                            if v.is_null() {
                                continue;
                            }
                            down = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "off" => {
                            if v.is_null() {
                                continue;
                            }
                            off = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "up" => {
                            if v.is_null() {
                                continue;
                            }
                            up = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "warning" => {
                            if v.is_null() {
                                continue;
                            }
                            warning = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = DeviceAttributesInterfaceStatuses {
                    down,
                    off,
                    up,
                    warning,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DeviceAttributesInterfaceStatusesVisitor)
    }
}

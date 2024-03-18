// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Container Image breakdown by supported platform.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ContainerImageFlavor {
    /// Time the platform-specific Container Image was built.
    #[serde(rename = "built_at")]
    pub built_at: Option<String>,
    /// Operating System architecture supported by the Container Image.
    #[serde(rename = "os_architecture")]
    pub os_architecture: Option<String>,
    /// Operating System name supported by the Container Image.
    #[serde(rename = "os_name")]
    pub os_name: Option<String>,
    /// Operating System version supported by the Container Image.
    #[serde(rename = "os_version")]
    pub os_version: Option<String>,
    /// Size of the platform-specific Container Image.
    #[serde(rename = "size")]
    pub size: Option<i64>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ContainerImageFlavor {
    pub fn new() -> ContainerImageFlavor {
        ContainerImageFlavor {
            built_at: None,
            os_architecture: None,
            os_name: None,
            os_version: None,
            size: None,
            _unparsed: false,
        }
    }

    pub fn built_at(mut self, value: String) -> Self {
        self.built_at = Some(value);
        self
    }

    pub fn os_architecture(mut self, value: String) -> Self {
        self.os_architecture = Some(value);
        self
    }

    pub fn os_name(mut self, value: String) -> Self {
        self.os_name = Some(value);
        self
    }

    pub fn os_version(mut self, value: String) -> Self {
        self.os_version = Some(value);
        self
    }

    pub fn size(mut self, value: i64) -> Self {
        self.size = Some(value);
        self
    }
}

impl Default for ContainerImageFlavor {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ContainerImageFlavor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ContainerImageFlavorVisitor;
        impl<'a> Visitor<'a> for ContainerImageFlavorVisitor {
            type Value = ContainerImageFlavor;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut built_at: Option<String> = None;
                let mut os_architecture: Option<String> = None;
                let mut os_name: Option<String> = None;
                let mut os_version: Option<String> = None;
                let mut size: Option<i64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "built_at" => {
                            if v.is_null() {
                                continue;
                            }
                            built_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "os_architecture" => {
                            if v.is_null() {
                                continue;
                            }
                            os_architecture =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "os_name" => {
                            if v.is_null() {
                                continue;
                            }
                            os_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "os_version" => {
                            if v.is_null() {
                                continue;
                            }
                            os_version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "size" => {
                            if v.is_null() {
                                continue;
                            }
                            size = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = ContainerImageFlavor {
                    built_at,
                    os_architecture,
                    os_name,
                    os_version,
                    size,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ContainerImageFlavorVisitor)
    }
}

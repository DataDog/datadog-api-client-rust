// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Agent install method.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct HostMetaInstallMethod {
    /// The installer version.
    #[serde(rename = "installer_version")]
    pub installer_version: Option<String>,
    /// Tool used to install the agent.
    #[serde(rename = "tool")]
    pub tool: Option<String>,
    /// The tool version.
    #[serde(rename = "tool_version")]
    pub tool_version: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl HostMetaInstallMethod {
    pub fn new() -> HostMetaInstallMethod {
        HostMetaInstallMethod {
            installer_version: None,
            tool: None,
            tool_version: None,
            _unparsed: false,
        }
    }

    pub fn installer_version(&mut self, value: String) -> &mut Self {
        self.installer_version = Some(value);
        self
    }

    pub fn tool(&mut self, value: String) -> &mut Self {
        self.tool = Some(value);
        self
    }

    pub fn tool_version(&mut self, value: String) -> &mut Self {
        self.tool_version = Some(value);
        self
    }
}

impl Default for HostMetaInstallMethod {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for HostMetaInstallMethod {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HostMetaInstallMethodVisitor;
        impl<'a> Visitor<'a> for HostMetaInstallMethodVisitor {
            type Value = HostMetaInstallMethod;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut installer_version: Option<String> = None;
                let mut tool: Option<String> = None;
                let mut tool_version: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "installer_version" => {
                            if v.is_null() {
                                continue;
                            }
                            installer_version =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tool" => {
                            if v.is_null() {
                                continue;
                            }
                            tool = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tool_version" => {
                            if v.is_null() {
                                continue;
                            }
                            tool_version =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = HostMetaInstallMethod {
                    installer_version,
                    tool,
                    tool_version,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(HostMetaInstallMethodVisitor)
    }
}

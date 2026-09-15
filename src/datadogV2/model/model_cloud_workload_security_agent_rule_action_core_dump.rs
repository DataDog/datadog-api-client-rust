// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The core dump action applied on the process matching the rule.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CloudWorkloadSecurityAgentRuleActionCoreDump {
    /// Whether the directory entry information is included in the core dump.
    #[serde(rename = "dentry")]
    pub dentry: Option<bool>,
    /// Whether the mount information is included in the core dump.
    #[serde(rename = "mount")]
    pub mount: Option<bool>,
    /// Whether the core dump is left uncompressed.
    #[serde(rename = "no_compression")]
    pub no_compression: Option<bool>,
    /// Whether the process memory is included in the core dump.
    #[serde(rename = "process")]
    pub process: Option<bool>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CloudWorkloadSecurityAgentRuleActionCoreDump {
    pub fn new() -> CloudWorkloadSecurityAgentRuleActionCoreDump {
        CloudWorkloadSecurityAgentRuleActionCoreDump {
            dentry: None,
            mount: None,
            no_compression: None,
            process: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn dentry(mut self, value: bool) -> Self {
        self.dentry = Some(value);
        self
    }

    pub fn mount(mut self, value: bool) -> Self {
        self.mount = Some(value);
        self
    }

    pub fn no_compression(mut self, value: bool) -> Self {
        self.no_compression = Some(value);
        self
    }

    pub fn process(mut self, value: bool) -> Self {
        self.process = Some(value);
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

impl Default for CloudWorkloadSecurityAgentRuleActionCoreDump {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CloudWorkloadSecurityAgentRuleActionCoreDump {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CloudWorkloadSecurityAgentRuleActionCoreDumpVisitor;
        impl<'a> Visitor<'a> for CloudWorkloadSecurityAgentRuleActionCoreDumpVisitor {
            type Value = CloudWorkloadSecurityAgentRuleActionCoreDump;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut dentry: Option<bool> = None;
                let mut mount: Option<bool> = None;
                let mut no_compression: Option<bool> = None;
                let mut process: Option<bool> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "dentry" => {
                            if v.is_null() {
                                continue;
                            }
                            dentry = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mount" => {
                            if v.is_null() {
                                continue;
                            }
                            mount = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "no_compression" => {
                            if v.is_null() {
                                continue;
                            }
                            no_compression =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "process" => {
                            if v.is_null() {
                                continue;
                            }
                            process = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = CloudWorkloadSecurityAgentRuleActionCoreDump {
                    dentry,
                    mount,
                    no_compression,
                    process,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CloudWorkloadSecurityAgentRuleActionCoreDumpVisitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A script that can be used to remediate the issue.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AgentHealthRemediationScript {
    /// The content of the script.
    #[serde(rename = "content")]
    pub content: Option<String>,
    /// The filename of the script.
    #[serde(rename = "filename")]
    pub filename: Option<String>,
    /// The language of the script.
    #[serde(rename = "language")]
    pub language: Option<String>,
    /// The language version used by the script.
    #[serde(rename = "language_version")]
    pub language_version: Option<String>,
    /// Whether the script requires root privileges.
    #[serde(rename = "requires_root")]
    pub requires_root: Option<bool>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AgentHealthRemediationScript {
    pub fn new() -> AgentHealthRemediationScript {
        AgentHealthRemediationScript {
            content: None,
            filename: None,
            language: None,
            language_version: None,
            requires_root: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn content(mut self, value: String) -> Self {
        self.content = Some(value);
        self
    }

    pub fn filename(mut self, value: String) -> Self {
        self.filename = Some(value);
        self
    }

    pub fn language(mut self, value: String) -> Self {
        self.language = Some(value);
        self
    }

    pub fn language_version(mut self, value: String) -> Self {
        self.language_version = Some(value);
        self
    }

    pub fn requires_root(mut self, value: bool) -> Self {
        self.requires_root = Some(value);
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

impl Default for AgentHealthRemediationScript {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AgentHealthRemediationScript {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AgentHealthRemediationScriptVisitor;
        impl<'a> Visitor<'a> for AgentHealthRemediationScriptVisitor {
            type Value = AgentHealthRemediationScript;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut content: Option<String> = None;
                let mut filename: Option<String> = None;
                let mut language: Option<String> = None;
                let mut language_version: Option<String> = None;
                let mut requires_root: Option<bool> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "content" => {
                            if v.is_null() {
                                continue;
                            }
                            content = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "filename" => {
                            if v.is_null() {
                                continue;
                            }
                            filename = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "language" => {
                            if v.is_null() {
                                continue;
                            }
                            language = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "language_version" => {
                            if v.is_null() {
                                continue;
                            }
                            language_version =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "requires_root" => {
                            if v.is_null() {
                                continue;
                            }
                            requires_root =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = AgentHealthRemediationScript {
                    content,
                    filename,
                    language,
                    language_version,
                    requires_root,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AgentHealthRemediationScriptVisitor)
    }
}

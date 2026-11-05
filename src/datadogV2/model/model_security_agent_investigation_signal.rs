// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Summary of the signal the investigation is about.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityAgentInvestigationSignal {
    /// Severity of the signal: `info`, `low`, `medium`, `high`, or `critical`.
    #[serde(rename = "severity")]
    pub severity: Option<String>,
    /// Sources of the signal.
    #[serde(rename = "sources")]
    pub sources: Option<Vec<String>>,
    /// MITRE ATT&CK tactic, from the signal's `tactic` tag.
    #[serde(rename = "tactic")]
    pub tactic: Option<String>,
    /// Tags of the signal, written `key:value`.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// MITRE ATT&CK technique, from the signal's `technique` tag.
    #[serde(rename = "technique")]
    pub technique: Option<String>,
    /// Time the signal was generated.
    #[serde(rename = "timestamp")]
    pub timestamp: Option<String>,
    /// Title of the signal.
    #[serde(rename = "title")]
    pub title: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityAgentInvestigationSignal {
    pub fn new() -> SecurityAgentInvestigationSignal {
        SecurityAgentInvestigationSignal {
            severity: None,
            sources: None,
            tactic: None,
            tags: None,
            technique: None,
            timestamp: None,
            title: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn severity(mut self, value: String) -> Self {
        self.severity = Some(value);
        self
    }

    pub fn sources(mut self, value: Vec<String>) -> Self {
        self.sources = Some(value);
        self
    }

    pub fn tactic(mut self, value: String) -> Self {
        self.tactic = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn technique(mut self, value: String) -> Self {
        self.technique = Some(value);
        self
    }

    pub fn timestamp(mut self, value: String) -> Self {
        self.timestamp = Some(value);
        self
    }

    pub fn title(mut self, value: String) -> Self {
        self.title = Some(value);
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

impl Default for SecurityAgentInvestigationSignal {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SecurityAgentInvestigationSignal {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityAgentInvestigationSignalVisitor;
        impl<'a> Visitor<'a> for SecurityAgentInvestigationSignalVisitor {
            type Value = SecurityAgentInvestigationSignal;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut severity: Option<String> = None;
                let mut sources: Option<Vec<String>> = None;
                let mut tactic: Option<String> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut technique: Option<String> = None;
                let mut timestamp: Option<String> = None;
                let mut title: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "severity" => {
                            if v.is_null() {
                                continue;
                            }
                            severity = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sources" => {
                            if v.is_null() {
                                continue;
                            }
                            sources = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tactic" => {
                            if v.is_null() {
                                continue;
                            }
                            tactic = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "technique" => {
                            if v.is_null() {
                                continue;
                            }
                            technique = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timestamp" => {
                            if v.is_null() {
                                continue;
                            }
                            timestamp = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "title" => {
                            if v.is_null() {
                                continue;
                            }
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SecurityAgentInvestigationSignal {
                    severity,
                    sources,
                    tactic,
                    tags,
                    technique,
                    timestamp,
                    title,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityAgentInvestigationSignalVisitor)
    }
}

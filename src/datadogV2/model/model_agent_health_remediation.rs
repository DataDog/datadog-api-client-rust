// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Remediation for the issue. Omitted when no remediation is available.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AgentHealthRemediation {
    /// A script that can be used to remediate the issue.
    #[serde(rename = "script")]
    pub script: Option<crate::datadogV2::model::AgentHealthRemediationScript>,
    /// Steps to remediate the issue.
    #[serde(rename = "steps")]
    pub steps: Option<Vec<crate::datadogV2::model::AgentHealthRemediationStep>>,
    /// A summary of how to remediate the issue.
    #[serde(rename = "summary")]
    pub summary: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AgentHealthRemediation {
    pub fn new() -> AgentHealthRemediation {
        AgentHealthRemediation {
            script: None,
            steps: None,
            summary: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn script(mut self, value: crate::datadogV2::model::AgentHealthRemediationScript) -> Self {
        self.script = Some(value);
        self
    }

    pub fn steps(
        mut self,
        value: Vec<crate::datadogV2::model::AgentHealthRemediationStep>,
    ) -> Self {
        self.steps = Some(value);
        self
    }

    pub fn summary(mut self, value: String) -> Self {
        self.summary = Some(value);
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

impl Default for AgentHealthRemediation {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AgentHealthRemediation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AgentHealthRemediationVisitor;
        impl<'a> Visitor<'a> for AgentHealthRemediationVisitor {
            type Value = AgentHealthRemediation;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut script: Option<crate::datadogV2::model::AgentHealthRemediationScript> =
                    None;
                let mut steps: Option<Vec<crate::datadogV2::model::AgentHealthRemediationStep>> =
                    None;
                let mut summary: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "script" => {
                            if v.is_null() {
                                continue;
                            }
                            script = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "steps" => {
                            if v.is_null() {
                                continue;
                            }
                            steps = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "summary" => {
                            if v.is_null() {
                                continue;
                            }
                            summary = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = AgentHealthRemediation {
                    script,
                    steps,
                    summary,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AgentHealthRemediationVisitor)
    }
}

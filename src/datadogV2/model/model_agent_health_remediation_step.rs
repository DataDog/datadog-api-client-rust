// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A step to remediate the issue.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AgentHealthRemediationStep {
    /// The order in which to perform the step.
    #[serde(rename = "order")]
    pub order: Option<i64>,
    /// The instructions for the step.
    #[serde(rename = "text")]
    pub text: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AgentHealthRemediationStep {
    pub fn new() -> AgentHealthRemediationStep {
        AgentHealthRemediationStep {
            order: None,
            text: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn order(mut self, value: i64) -> Self {
        self.order = Some(value);
        self
    }

    pub fn text(mut self, value: String) -> Self {
        self.text = Some(value);
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

impl Default for AgentHealthRemediationStep {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AgentHealthRemediationStep {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AgentHealthRemediationStepVisitor;
        impl<'a> Visitor<'a> for AgentHealthRemediationStepVisitor {
            type Value = AgentHealthRemediationStep;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut order: Option<i64> = None;
                let mut text: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "order" => {
                            if v.is_null() {
                                continue;
                            }
                            order = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "text" => {
                            if v.is_null() {
                                continue;
                            }
                            text = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = AgentHealthRemediationStep {
                    order,
                    text,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AgentHealthRemediationStepVisitor)
    }
}

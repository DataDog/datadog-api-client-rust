// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Rule failure details.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DeploymentGateRuleFailures {
    /// Names of faulty APM resources.
    #[serde(rename = "faulty_apm_resources")]
    pub faulty_apm_resources: Vec<String>,
    #[serde(rename = "monitors")]
    pub monitors: Vec<crate::datadogV2::model::DeploymentGateRuleFailureMonitor>,
    #[serde(rename = "narratives")]
    pub narratives: Vec<crate::datadogV2::model::DeploymentGateRuleFailureNarrative>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DeploymentGateRuleFailures {
    pub fn new(
        faulty_apm_resources: Vec<String>,
        monitors: Vec<crate::datadogV2::model::DeploymentGateRuleFailureMonitor>,
        narratives: Vec<crate::datadogV2::model::DeploymentGateRuleFailureNarrative>,
    ) -> DeploymentGateRuleFailures {
        DeploymentGateRuleFailures {
            faulty_apm_resources,
            monitors,
            narratives,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for DeploymentGateRuleFailures {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DeploymentGateRuleFailuresVisitor;
        impl<'a> Visitor<'a> for DeploymentGateRuleFailuresVisitor {
            type Value = DeploymentGateRuleFailures;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut faulty_apm_resources: Option<Vec<String>> = None;
                let mut monitors: Option<
                    Vec<crate::datadogV2::model::DeploymentGateRuleFailureMonitor>,
                > = None;
                let mut narratives: Option<
                    Vec<crate::datadogV2::model::DeploymentGateRuleFailureNarrative>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "faulty_apm_resources" => {
                            faulty_apm_resources =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "monitors" => {
                            monitors = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "narratives" => {
                            narratives = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let faulty_apm_resources = faulty_apm_resources
                    .ok_or_else(|| M::Error::missing_field("faulty_apm_resources"))?;
                let monitors = monitors.ok_or_else(|| M::Error::missing_field("monitors"))?;
                let narratives = narratives.ok_or_else(|| M::Error::missing_field("narratives"))?;

                let content = DeploymentGateRuleFailures {
                    faulty_apm_resources,
                    monitors,
                    narratives,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DeploymentGateRuleFailuresVisitor)
    }
}

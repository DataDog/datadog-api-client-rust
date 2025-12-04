// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for agent information.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FleetAgentInfoAttributes {
    /// Detailed information about a Datadog Agent.
    #[serde(rename = "agent_infos")]
    pub agent_infos: Option<crate::datadogV2::model::FleetAgentInfoDetails>,
    /// Configuration information organized by layers.
    #[serde(rename = "configuration_files")]
    pub configuration_files: Option<crate::datadogV2::model::FleetConfigurationLayer>,
    /// List of detected integrations.
    #[serde(rename = "detected_integrations")]
    pub detected_integrations: Option<Vec<crate::datadogV2::model::FleetDetectedIntegration>>,
    /// Integrations organized by their status.
    #[serde(rename = "integrations")]
    pub integrations: Option<crate::datadogV2::model::FleetIntegrationsByStatus>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FleetAgentInfoAttributes {
    pub fn new() -> FleetAgentInfoAttributes {
        FleetAgentInfoAttributes {
            agent_infos: None,
            configuration_files: None,
            detected_integrations: None,
            integrations: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn agent_infos(mut self, value: crate::datadogV2::model::FleetAgentInfoDetails) -> Self {
        self.agent_infos = Some(value);
        self
    }

    pub fn configuration_files(
        mut self,
        value: crate::datadogV2::model::FleetConfigurationLayer,
    ) -> Self {
        self.configuration_files = Some(value);
        self
    }

    pub fn detected_integrations(
        mut self,
        value: Vec<crate::datadogV2::model::FleetDetectedIntegration>,
    ) -> Self {
        self.detected_integrations = Some(value);
        self
    }

    pub fn integrations(
        mut self,
        value: crate::datadogV2::model::FleetIntegrationsByStatus,
    ) -> Self {
        self.integrations = Some(value);
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

impl Default for FleetAgentInfoAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for FleetAgentInfoAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FleetAgentInfoAttributesVisitor;
        impl<'a> Visitor<'a> for FleetAgentInfoAttributesVisitor {
            type Value = FleetAgentInfoAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut agent_infos: Option<crate::datadogV2::model::FleetAgentInfoDetails> = None;
                let mut configuration_files: Option<
                    crate::datadogV2::model::FleetConfigurationLayer,
                > = None;
                let mut detected_integrations: Option<
                    Vec<crate::datadogV2::model::FleetDetectedIntegration>,
                > = None;
                let mut integrations: Option<crate::datadogV2::model::FleetIntegrationsByStatus> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "agent_infos" => {
                            if v.is_null() {
                                continue;
                            }
                            agent_infos =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "configuration_files" => {
                            if v.is_null() {
                                continue;
                            }
                            configuration_files =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "detected_integrations" => {
                            if v.is_null() {
                                continue;
                            }
                            detected_integrations =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "integrations" => {
                            if v.is_null() {
                                continue;
                            }
                            integrations =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = FleetAgentInfoAttributes {
                    agent_infos,
                    configuration_files,
                    detected_integrations,
                    integrations,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FleetAgentInfoAttributesVisitor)
    }
}

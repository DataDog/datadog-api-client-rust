// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for the v2 agent detail response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FleetAgentDetailV2Attributes {
    /// Detailed information about a Datadog Agent.
    #[serde(rename = "agent_infos")]
    pub agent_infos: crate::datadogV2::model::FleetAgentInfoDetailsV2,
    /// Configuration details for an agent, organized by configuration layer.
    #[serde(rename = "configuration_files")]
    pub configuration_files: Option<crate::datadogV2::model::FleetAgentConfigurationFilesV2>,
    /// Integrations organized by their status.
    #[serde(rename = "integrations")]
    pub integrations: Option<crate::datadogV2::model::FleetIntegrationsByStatusV2>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FleetAgentDetailV2Attributes {
    pub fn new(
        agent_infos: crate::datadogV2::model::FleetAgentInfoDetailsV2,
    ) -> FleetAgentDetailV2Attributes {
        FleetAgentDetailV2Attributes {
            agent_infos,
            configuration_files: None,
            integrations: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn configuration_files(
        mut self,
        value: crate::datadogV2::model::FleetAgentConfigurationFilesV2,
    ) -> Self {
        self.configuration_files = Some(value);
        self
    }

    pub fn integrations(
        mut self,
        value: crate::datadogV2::model::FleetIntegrationsByStatusV2,
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

impl<'de> Deserialize<'de> for FleetAgentDetailV2Attributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FleetAgentDetailV2AttributesVisitor;
        impl<'a> Visitor<'a> for FleetAgentDetailV2AttributesVisitor {
            type Value = FleetAgentDetailV2Attributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut agent_infos: Option<crate::datadogV2::model::FleetAgentInfoDetailsV2> =
                    None;
                let mut configuration_files: Option<
                    crate::datadogV2::model::FleetAgentConfigurationFilesV2,
                > = None;
                let mut integrations: Option<crate::datadogV2::model::FleetIntegrationsByStatusV2> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "agent_infos" => {
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
                let agent_infos =
                    agent_infos.ok_or_else(|| M::Error::missing_field("agent_infos"))?;

                let content = FleetAgentDetailV2Attributes {
                    agent_infos,
                    configuration_files,
                    integrations,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FleetAgentDetailV2AttributesVisitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Configuration details for an agent, organized by configuration layer.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FleetAgentConfigurationFilesV2 {
    /// Configuration information organized by layers.
    #[serde(rename = "agent_configuration")]
    pub agent_configuration: Option<crate::datadogV2::model::FleetConfigurationLayer>,
    /// Configuration information organized by layers.
    #[serde(rename = "application_monitoring_configuration")]
    pub application_monitoring_configuration:
        Option<crate::datadogV2::model::FleetConfigurationLayer>,
    /// The unique agent key identifier.
    #[serde(rename = "datadog_agent_key")]
    pub datadog_agent_key: Option<String>,
    /// Configuration for OpenTelemetry collectors associated with the agent. Present only when the agent has associated OpenTelemetry collectors.
    #[serde(rename = "otel_collectors_configuration")]
    pub otel_collectors_configuration:
        Option<Vec<crate::datadogV2::model::FleetOtelCollectorConfigurationV2>>,
    /// Configuration information organized by layers.
    #[serde(rename = "security_agent_configuration")]
    pub security_agent_configuration: Option<crate::datadogV2::model::FleetConfigurationLayer>,
    /// Configuration information organized by layers.
    #[serde(rename = "system_probe_configuration")]
    pub system_probe_configuration: Option<crate::datadogV2::model::FleetConfigurationLayer>,
    /// The configuration version.
    #[serde(rename = "version")]
    pub version: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FleetAgentConfigurationFilesV2 {
    pub fn new() -> FleetAgentConfigurationFilesV2 {
        FleetAgentConfigurationFilesV2 {
            agent_configuration: None,
            application_monitoring_configuration: None,
            datadog_agent_key: None,
            otel_collectors_configuration: None,
            security_agent_configuration: None,
            system_probe_configuration: None,
            version: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn agent_configuration(
        mut self,
        value: crate::datadogV2::model::FleetConfigurationLayer,
    ) -> Self {
        self.agent_configuration = Some(value);
        self
    }

    pub fn application_monitoring_configuration(
        mut self,
        value: crate::datadogV2::model::FleetConfigurationLayer,
    ) -> Self {
        self.application_monitoring_configuration = Some(value);
        self
    }

    pub fn datadog_agent_key(mut self, value: String) -> Self {
        self.datadog_agent_key = Some(value);
        self
    }

    pub fn otel_collectors_configuration(
        mut self,
        value: Vec<crate::datadogV2::model::FleetOtelCollectorConfigurationV2>,
    ) -> Self {
        self.otel_collectors_configuration = Some(value);
        self
    }

    pub fn security_agent_configuration(
        mut self,
        value: crate::datadogV2::model::FleetConfigurationLayer,
    ) -> Self {
        self.security_agent_configuration = Some(value);
        self
    }

    pub fn system_probe_configuration(
        mut self,
        value: crate::datadogV2::model::FleetConfigurationLayer,
    ) -> Self {
        self.system_probe_configuration = Some(value);
        self
    }

    pub fn version(mut self, value: String) -> Self {
        self.version = Some(value);
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

impl Default for FleetAgentConfigurationFilesV2 {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for FleetAgentConfigurationFilesV2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FleetAgentConfigurationFilesV2Visitor;
        impl<'a> Visitor<'a> for FleetAgentConfigurationFilesV2Visitor {
            type Value = FleetAgentConfigurationFilesV2;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut agent_configuration: Option<
                    crate::datadogV2::model::FleetConfigurationLayer,
                > = None;
                let mut application_monitoring_configuration: Option<
                    crate::datadogV2::model::FleetConfigurationLayer,
                > = None;
                let mut datadog_agent_key: Option<String> = None;
                let mut otel_collectors_configuration: Option<
                    Vec<crate::datadogV2::model::FleetOtelCollectorConfigurationV2>,
                > = None;
                let mut security_agent_configuration: Option<
                    crate::datadogV2::model::FleetConfigurationLayer,
                > = None;
                let mut system_probe_configuration: Option<
                    crate::datadogV2::model::FleetConfigurationLayer,
                > = None;
                let mut version: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "agent_configuration" => {
                            if v.is_null() {
                                continue;
                            }
                            agent_configuration =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "application_monitoring_configuration" => {
                            if v.is_null() {
                                continue;
                            }
                            application_monitoring_configuration =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "datadog_agent_key" => {
                            if v.is_null() {
                                continue;
                            }
                            datadog_agent_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "otel_collectors_configuration" => {
                            if v.is_null() {
                                continue;
                            }
                            otel_collectors_configuration =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "security_agent_configuration" => {
                            if v.is_null() {
                                continue;
                            }
                            security_agent_configuration =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "system_probe_configuration" => {
                            if v.is_null() {
                                continue;
                            }
                            system_probe_configuration =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "version" => {
                            if v.is_null() {
                                continue;
                            }
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = FleetAgentConfigurationFilesV2 {
                    agent_configuration,
                    application_monitoring_configuration,
                    datadog_agent_key,
                    otel_collectors_configuration,
                    security_agent_configuration,
                    system_probe_configuration,
                    version,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FleetAgentConfigurationFilesV2Visitor)
    }
}

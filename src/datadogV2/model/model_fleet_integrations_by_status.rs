// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Integrations organized by their status.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FleetIntegrationsByStatus {
    /// Configuration files for integrations.
    #[serde(rename = "configuration_files")]
    pub configuration_files: Option<Vec<crate::datadogV2::model::FleetConfigurationFile>>,
    /// The unique agent key identifier.
    #[serde(rename = "datadog_agent_key")]
    pub datadog_agent_key: Option<String>,
    /// Integrations with errors.
    #[serde(rename = "error_integrations")]
    pub error_integrations: Option<Vec<crate::datadogV2::model::FleetIntegrationDetails>>,
    /// Detected but not configured integrations.
    #[serde(rename = "missing_integrations")]
    pub missing_integrations: Option<Vec<crate::datadogV2::model::FleetDetectedIntegration>>,
    /// Integrations with warnings.
    #[serde(rename = "warning_integrations")]
    pub warning_integrations: Option<Vec<crate::datadogV2::model::FleetIntegrationDetails>>,
    /// Integrations that are working correctly.
    #[serde(rename = "working_integrations")]
    pub working_integrations: Option<Vec<crate::datadogV2::model::FleetIntegrationDetails>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FleetIntegrationsByStatus {
    pub fn new() -> FleetIntegrationsByStatus {
        FleetIntegrationsByStatus {
            configuration_files: None,
            datadog_agent_key: None,
            error_integrations: None,
            missing_integrations: None,
            warning_integrations: None,
            working_integrations: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn configuration_files(
        mut self,
        value: Vec<crate::datadogV2::model::FleetConfigurationFile>,
    ) -> Self {
        self.configuration_files = Some(value);
        self
    }

    pub fn datadog_agent_key(mut self, value: String) -> Self {
        self.datadog_agent_key = Some(value);
        self
    }

    pub fn error_integrations(
        mut self,
        value: Vec<crate::datadogV2::model::FleetIntegrationDetails>,
    ) -> Self {
        self.error_integrations = Some(value);
        self
    }

    pub fn missing_integrations(
        mut self,
        value: Vec<crate::datadogV2::model::FleetDetectedIntegration>,
    ) -> Self {
        self.missing_integrations = Some(value);
        self
    }

    pub fn warning_integrations(
        mut self,
        value: Vec<crate::datadogV2::model::FleetIntegrationDetails>,
    ) -> Self {
        self.warning_integrations = Some(value);
        self
    }

    pub fn working_integrations(
        mut self,
        value: Vec<crate::datadogV2::model::FleetIntegrationDetails>,
    ) -> Self {
        self.working_integrations = Some(value);
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

impl Default for FleetIntegrationsByStatus {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for FleetIntegrationsByStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FleetIntegrationsByStatusVisitor;
        impl<'a> Visitor<'a> for FleetIntegrationsByStatusVisitor {
            type Value = FleetIntegrationsByStatus;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut configuration_files: Option<
                    Vec<crate::datadogV2::model::FleetConfigurationFile>,
                > = None;
                let mut datadog_agent_key: Option<String> = None;
                let mut error_integrations: Option<
                    Vec<crate::datadogV2::model::FleetIntegrationDetails>,
                > = None;
                let mut missing_integrations: Option<
                    Vec<crate::datadogV2::model::FleetDetectedIntegration>,
                > = None;
                let mut warning_integrations: Option<
                    Vec<crate::datadogV2::model::FleetIntegrationDetails>,
                > = None;
                let mut working_integrations: Option<
                    Vec<crate::datadogV2::model::FleetIntegrationDetails>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "configuration_files" => {
                            if v.is_null() {
                                continue;
                            }
                            configuration_files =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "datadog_agent_key" => {
                            if v.is_null() {
                                continue;
                            }
                            datadog_agent_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "error_integrations" => {
                            if v.is_null() {
                                continue;
                            }
                            error_integrations =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "missing_integrations" => {
                            if v.is_null() {
                                continue;
                            }
                            missing_integrations =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "warning_integrations" => {
                            if v.is_null() {
                                continue;
                            }
                            warning_integrations =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "working_integrations" => {
                            if v.is_null() {
                                continue;
                            }
                            working_integrations =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = FleetIntegrationsByStatus {
                    configuration_files,
                    datadog_agent_key,
                    error_integrations,
                    missing_integrations,
                    warning_integrations,
                    working_integrations,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FleetIntegrationsByStatusVisitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Per-host status entry for a deployment.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FleetDeploymentV2DetailAgent {
    /// Error message if the deployment failed on this host.
    #[serde(rename = "error")]
    pub error: Option<String>,
    /// Hostname of the agent.
    #[serde(rename = "hostname")]
    pub hostname: Option<String>,
    /// Name of the step currently executing on this host.
    #[serde(rename = "running_step")]
    pub running_step: Option<String>,
    /// Deployment status for this host (for example, "pending", "running", "succeeded", "failed").
    #[serde(rename = "status")]
    pub status: Option<String>,
    /// Additional details about the current deployment status on this host.
    #[serde(rename = "status_details")]
    pub status_details: Option<String>,
    /// Package version details for this host.
    #[serde(rename = "versions")]
    pub versions: Option<Vec<crate::datadogV2::model::FleetDeploymentHostPackage>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FleetDeploymentV2DetailAgent {
    pub fn new() -> FleetDeploymentV2DetailAgent {
        FleetDeploymentV2DetailAgent {
            error: None,
            hostname: None,
            running_step: None,
            status: None,
            status_details: None,
            versions: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn error(mut self, value: String) -> Self {
        self.error = Some(value);
        self
    }

    pub fn hostname(mut self, value: String) -> Self {
        self.hostname = Some(value);
        self
    }

    pub fn running_step(mut self, value: String) -> Self {
        self.running_step = Some(value);
        self
    }

    pub fn status(mut self, value: String) -> Self {
        self.status = Some(value);
        self
    }

    pub fn status_details(mut self, value: String) -> Self {
        self.status_details = Some(value);
        self
    }

    pub fn versions(
        mut self,
        value: Vec<crate::datadogV2::model::FleetDeploymentHostPackage>,
    ) -> Self {
        self.versions = Some(value);
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

impl Default for FleetDeploymentV2DetailAgent {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for FleetDeploymentV2DetailAgent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FleetDeploymentV2DetailAgentVisitor;
        impl<'a> Visitor<'a> for FleetDeploymentV2DetailAgentVisitor {
            type Value = FleetDeploymentV2DetailAgent;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut error: Option<String> = None;
                let mut hostname: Option<String> = None;
                let mut running_step: Option<String> = None;
                let mut status: Option<String> = None;
                let mut status_details: Option<String> = None;
                let mut versions: Option<Vec<crate::datadogV2::model::FleetDeploymentHostPackage>> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "error" => {
                            if v.is_null() {
                                continue;
                            }
                            error = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "hostname" => {
                            if v.is_null() {
                                continue;
                            }
                            hostname = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "running_step" => {
                            if v.is_null() {
                                continue;
                            }
                            running_step =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status_details" => {
                            if v.is_null() {
                                continue;
                            }
                            status_details =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "versions" => {
                            if v.is_null() {
                                continue;
                            }
                            versions = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = FleetDeploymentV2DetailAgent {
                    error,
                    hostname,
                    running_step,
                    status,
                    status_details,
                    versions,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FleetDeploymentV2DetailAgentVisitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Package version information for a host, showing the initial version before deployment,
/// the target version to deploy, and the current version on the host.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FleetDeploymentHostPackage {
    /// The current version of the package on the host.
    #[serde(rename = "current_version")]
    pub current_version: Option<String>,
    /// The initial version of the package on the host before the deployment started.
    #[serde(rename = "initial_version")]
    pub initial_version: Option<String>,
    /// The name of the package.
    #[serde(rename = "package_name")]
    pub package_name: Option<String>,
    /// The target version that the deployment is attempting to install.
    #[serde(rename = "target_version")]
    pub target_version: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FleetDeploymentHostPackage {
    pub fn new() -> FleetDeploymentHostPackage {
        FleetDeploymentHostPackage {
            current_version: None,
            initial_version: None,
            package_name: None,
            target_version: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn current_version(mut self, value: String) -> Self {
        self.current_version = Some(value);
        self
    }

    pub fn initial_version(mut self, value: String) -> Self {
        self.initial_version = Some(value);
        self
    }

    pub fn package_name(mut self, value: String) -> Self {
        self.package_name = Some(value);
        self
    }

    pub fn target_version(mut self, value: String) -> Self {
        self.target_version = Some(value);
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

impl Default for FleetDeploymentHostPackage {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for FleetDeploymentHostPackage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FleetDeploymentHostPackageVisitor;
        impl<'a> Visitor<'a> for FleetDeploymentHostPackageVisitor {
            type Value = FleetDeploymentHostPackage;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut current_version: Option<String> = None;
                let mut initial_version: Option<String> = None;
                let mut package_name: Option<String> = None;
                let mut target_version: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "current_version" => {
                            if v.is_null() {
                                continue;
                            }
                            current_version =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "initial_version" => {
                            if v.is_null() {
                                continue;
                            }
                            initial_version =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "package_name" => {
                            if v.is_null() {
                                continue;
                            }
                            package_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "target_version" => {
                            if v.is_null() {
                                continue;
                            }
                            target_version =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = FleetDeploymentHostPackage {
                    current_version,
                    initial_version,
                    package_name,
                    target_version,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FleetDeploymentHostPackageVisitor)
    }
}

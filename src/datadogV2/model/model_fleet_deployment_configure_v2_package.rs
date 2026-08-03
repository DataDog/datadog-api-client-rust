// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A package and its target version to additionally deploy alongside a configuration change.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FleetDeploymentConfigureV2Package {
    /// APM auto-instrumentation mode to enable for this package, if applicable.
    #[serde(rename = "apm_instrumentation")]
    pub apm_instrumentation: Option<String>,
    /// The name of the package to deploy.
    #[serde(rename = "name")]
    pub name: String,
    /// The target version of the package to deploy.
    #[serde(rename = "version")]
    pub version: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FleetDeploymentConfigureV2Package {
    pub fn new(name: String, version: String) -> FleetDeploymentConfigureV2Package {
        FleetDeploymentConfigureV2Package {
            apm_instrumentation: None,
            name,
            version,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn apm_instrumentation(mut self, value: String) -> Self {
        self.apm_instrumentation = Some(value);
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

impl<'de> Deserialize<'de> for FleetDeploymentConfigureV2Package {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FleetDeploymentConfigureV2PackageVisitor;
        impl<'a> Visitor<'a> for FleetDeploymentConfigureV2PackageVisitor {
            type Value = FleetDeploymentConfigureV2Package;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut apm_instrumentation: Option<String> = None;
                let mut name: Option<String> = None;
                let mut version: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "apm_instrumentation" => {
                            if v.is_null() {
                                continue;
                            }
                            apm_instrumentation =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "version" => {
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let version = version.ok_or_else(|| M::Error::missing_field("version"))?;

                let content = FleetDeploymentConfigureV2Package {
                    apm_instrumentation,
                    name,
                    version,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FleetDeploymentConfigureV2PackageVisitor)
    }
}

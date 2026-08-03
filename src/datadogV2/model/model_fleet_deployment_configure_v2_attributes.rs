// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for creating a new v2 configuration deployment.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FleetDeploymentConfigureV2Attributes {
    /// Ordered list of configuration file operations to perform on the target hosts.
    #[serde(rename = "config_operations")]
    pub config_operations: Vec<crate::datadogV2::model::FleetDeploymentOperation>,
    /// Set to `true` to validate the configuration and resolve target hosts and packages
    /// without deploying anything. Returns a 200 with the validation result instead of
    /// creating and starting a real deployment.
    #[serde(rename = "dry_run")]
    pub dry_run: Option<bool>,
    /// Query used to filter and select target hosts for the deployment. Uses the Datadog query syntax.
    #[serde(rename = "filter_query")]
    pub filter_query: String,
    /// List of packages and their target versions to additionally deploy alongside
    /// the configuration change.
    #[serde(rename = "target_packages")]
    pub target_packages: Option<Vec<crate::datadogV2::model::FleetDeploymentConfigureV2Package>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FleetDeploymentConfigureV2Attributes {
    pub fn new(
        config_operations: Vec<crate::datadogV2::model::FleetDeploymentOperation>,
        filter_query: String,
    ) -> FleetDeploymentConfigureV2Attributes {
        FleetDeploymentConfigureV2Attributes {
            config_operations,
            dry_run: None,
            filter_query,
            target_packages: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn dry_run(mut self, value: bool) -> Self {
        self.dry_run = Some(value);
        self
    }

    pub fn target_packages(
        mut self,
        value: Vec<crate::datadogV2::model::FleetDeploymentConfigureV2Package>,
    ) -> Self {
        self.target_packages = Some(value);
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

impl<'de> Deserialize<'de> for FleetDeploymentConfigureV2Attributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FleetDeploymentConfigureV2AttributesVisitor;
        impl<'a> Visitor<'a> for FleetDeploymentConfigureV2AttributesVisitor {
            type Value = FleetDeploymentConfigureV2Attributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut config_operations: Option<
                    Vec<crate::datadogV2::model::FleetDeploymentOperation>,
                > = None;
                let mut dry_run: Option<bool> = None;
                let mut filter_query: Option<String> = None;
                let mut target_packages: Option<
                    Vec<crate::datadogV2::model::FleetDeploymentConfigureV2Package>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "config_operations" => {
                            config_operations =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "dry_run" => {
                            if v.is_null() {
                                continue;
                            }
                            dry_run = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "filter_query" => {
                            filter_query =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "target_packages" => {
                            if v.is_null() {
                                continue;
                            }
                            target_packages =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let config_operations = config_operations
                    .ok_or_else(|| M::Error::missing_field("config_operations"))?;
                let filter_query =
                    filter_query.ok_or_else(|| M::Error::missing_field("filter_query"))?;

                let content = FleetDeploymentConfigureV2Attributes {
                    config_operations,
                    dry_run,
                    filter_query,
                    target_packages,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FleetDeploymentConfigureV2AttributesVisitor)
    }
}

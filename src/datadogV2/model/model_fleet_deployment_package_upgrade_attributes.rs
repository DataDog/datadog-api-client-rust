// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for creating a new package upgrade deployment.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FleetDeploymentPackageUpgradeAttributes {
    /// Query used to filter and select target hosts for the deployment. Uses the Datadog query syntax.
    #[serde(rename = "filter_query")]
    pub filter_query: Option<String>,
    /// List of packages and their target versions to deploy to the selected hosts.
    #[serde(rename = "target_packages")]
    pub target_packages: Vec<crate::datadogV2::model::FleetDeploymentPackage>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FleetDeploymentPackageUpgradeAttributes {
    pub fn new(
        target_packages: Vec<crate::datadogV2::model::FleetDeploymentPackage>,
    ) -> FleetDeploymentPackageUpgradeAttributes {
        FleetDeploymentPackageUpgradeAttributes {
            filter_query: None,
            target_packages,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn filter_query(mut self, value: String) -> Self {
        self.filter_query = Some(value);
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

impl<'de> Deserialize<'de> for FleetDeploymentPackageUpgradeAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FleetDeploymentPackageUpgradeAttributesVisitor;
        impl<'a> Visitor<'a> for FleetDeploymentPackageUpgradeAttributesVisitor {
            type Value = FleetDeploymentPackageUpgradeAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut filter_query: Option<String> = None;
                let mut target_packages: Option<
                    Vec<crate::datadogV2::model::FleetDeploymentPackage>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "filter_query" => {
                            if v.is_null() {
                                continue;
                            }
                            filter_query =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "target_packages" => {
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
                let target_packages =
                    target_packages.ok_or_else(|| M::Error::missing_field("target_packages"))?;

                let content = FleetDeploymentPackageUpgradeAttributes {
                    filter_query,
                    target_packages,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FleetDeploymentPackageUpgradeAttributesVisitor)
    }
}

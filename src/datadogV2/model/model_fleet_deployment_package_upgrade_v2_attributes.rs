// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for creating a new v2 package upgrade deployment.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FleetDeploymentPackageUpgradeV2Attributes {
    /// Query used to filter and select target hosts for the deployment. Uses the Datadog query syntax.
    #[serde(rename = "filter_query")]
    pub filter_query: String,
    /// List of packages and their target versions to deploy to the selected hosts.
    #[serde(rename = "target_packages")]
    pub target_packages: Vec<crate::datadogV2::model::FleetDeploymentPackage>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FleetDeploymentPackageUpgradeV2Attributes {
    pub fn new(
        filter_query: String,
        target_packages: Vec<crate::datadogV2::model::FleetDeploymentPackage>,
    ) -> FleetDeploymentPackageUpgradeV2Attributes {
        FleetDeploymentPackageUpgradeV2Attributes {
            filter_query,
            target_packages,
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

impl<'de> Deserialize<'de> for FleetDeploymentPackageUpgradeV2Attributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FleetDeploymentPackageUpgradeV2AttributesVisitor;
        impl<'a> Visitor<'a> for FleetDeploymentPackageUpgradeV2AttributesVisitor {
            type Value = FleetDeploymentPackageUpgradeV2Attributes;

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
                let filter_query =
                    filter_query.ok_or_else(|| M::Error::missing_field("filter_query"))?;
                let target_packages =
                    target_packages.ok_or_else(|| M::Error::missing_field("target_packages"))?;

                let content = FleetDeploymentPackageUpgradeV2Attributes {
                    filter_query,
                    target_packages,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FleetDeploymentPackageUpgradeV2AttributesVisitor)
    }
}

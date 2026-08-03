// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a configuration deployment dry-run response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FleetDeploymentConfigureV2DryRunAttributes {
    /// Validation result of a configuration deployment dry run.
    #[serde(rename = "dry_run")]
    pub dry_run: Option<crate::datadogV2::model::FleetDeploymentConfigureV2DryRunResult>,
    /// Query used to filter and select target hosts for the deployment.
    #[serde(rename = "query")]
    pub query: Option<String>,
    /// Total number of hosts targeted by the dry run.
    #[serde(rename = "total_hosts")]
    pub total_hosts: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FleetDeploymentConfigureV2DryRunAttributes {
    pub fn new() -> FleetDeploymentConfigureV2DryRunAttributes {
        FleetDeploymentConfigureV2DryRunAttributes {
            dry_run: None,
            query: None,
            total_hosts: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn dry_run(
        mut self,
        value: crate::datadogV2::model::FleetDeploymentConfigureV2DryRunResult,
    ) -> Self {
        self.dry_run = Some(value);
        self
    }

    pub fn query(mut self, value: String) -> Self {
        self.query = Some(value);
        self
    }

    pub fn total_hosts(mut self, value: i64) -> Self {
        self.total_hosts = Some(value);
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

impl Default for FleetDeploymentConfigureV2DryRunAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for FleetDeploymentConfigureV2DryRunAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FleetDeploymentConfigureV2DryRunAttributesVisitor;
        impl<'a> Visitor<'a> for FleetDeploymentConfigureV2DryRunAttributesVisitor {
            type Value = FleetDeploymentConfigureV2DryRunAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut dry_run: Option<
                    crate::datadogV2::model::FleetDeploymentConfigureV2DryRunResult,
                > = None;
                let mut query: Option<String> = None;
                let mut total_hosts: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "dry_run" => {
                            if v.is_null() {
                                continue;
                            }
                            dry_run = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query" => {
                            if v.is_null() {
                                continue;
                            }
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_hosts" => {
                            if v.is_null() {
                                continue;
                            }
                            total_hosts =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = FleetDeploymentConfigureV2DryRunAttributes {
                    dry_run,
                    query,
                    total_hosts,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FleetDeploymentConfigureV2DryRunAttributesVisitor)
    }
}

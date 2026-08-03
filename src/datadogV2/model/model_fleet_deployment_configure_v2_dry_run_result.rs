// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Validation result of a configuration deployment dry run.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FleetDeploymentConfigureV2DryRunResult {
    /// Whether the configuration passed schema validation.
    #[serde(rename = "config_validated")]
    pub config_validated: Option<bool>,
    /// Breakdown of ineligible host counts by reason. Only includes reasons with a
    /// non-zero count. Absent from the response when no targeted host is ineligible.
    #[serde(rename = "non_upgradable_by_reason")]
    pub non_upgradable_by_reason: Option<std::collections::BTreeMap<String, i64>>,
    /// Number of targeted hosts that are not eligible to receive this configuration.
    #[serde(rename = "non_upgradable_hosts")]
    pub non_upgradable_hosts: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FleetDeploymentConfigureV2DryRunResult {
    pub fn new() -> FleetDeploymentConfigureV2DryRunResult {
        FleetDeploymentConfigureV2DryRunResult {
            config_validated: None,
            non_upgradable_by_reason: None,
            non_upgradable_hosts: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn config_validated(mut self, value: bool) -> Self {
        self.config_validated = Some(value);
        self
    }

    pub fn non_upgradable_by_reason(
        mut self,
        value: std::collections::BTreeMap<String, i64>,
    ) -> Self {
        self.non_upgradable_by_reason = Some(value);
        self
    }

    pub fn non_upgradable_hosts(mut self, value: i64) -> Self {
        self.non_upgradable_hosts = Some(value);
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

impl Default for FleetDeploymentConfigureV2DryRunResult {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for FleetDeploymentConfigureV2DryRunResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FleetDeploymentConfigureV2DryRunResultVisitor;
        impl<'a> Visitor<'a> for FleetDeploymentConfigureV2DryRunResultVisitor {
            type Value = FleetDeploymentConfigureV2DryRunResult;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut config_validated: Option<bool> = None;
                let mut non_upgradable_by_reason: Option<std::collections::BTreeMap<String, i64>> =
                    None;
                let mut non_upgradable_hosts: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "config_validated" => {
                            if v.is_null() {
                                continue;
                            }
                            config_validated =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "non_upgradable_by_reason" => {
                            if v.is_null() {
                                continue;
                            }
                            non_upgradable_by_reason =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "non_upgradable_hosts" => {
                            if v.is_null() {
                                continue;
                            }
                            non_upgradable_hosts =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = FleetDeploymentConfigureV2DryRunResult {
                    config_validated,
                    non_upgradable_by_reason,
                    non_upgradable_hosts,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FleetDeploymentConfigureV2DryRunResultVisitor)
    }
}

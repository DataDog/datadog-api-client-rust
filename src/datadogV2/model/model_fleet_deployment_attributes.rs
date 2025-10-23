// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a deployment in the response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FleetDeploymentAttributes {
    /// Ordered list of configuration file operations to perform on the target hosts.
    #[serde(rename = "config_operations")]
    pub config_operations: Option<Vec<crate::datadogV2::model::FleetDeploymentOperation>>,
    /// Estimated completion time of the deployment as a Unix timestamp (seconds since epoch).
    #[serde(rename = "estimated_end_time_unix")]
    pub estimated_end_time_unix: Option<i64>,
    /// Query used to filter and select target hosts for the deployment. Uses the Datadog query syntax.
    #[serde(rename = "filter_query")]
    pub filter_query: Option<String>,
    /// Current high-level status of the deployment (for example, "pending", "running", "completed", "failed").
    #[serde(rename = "high_level_status")]
    pub high_level_status: Option<String>,
    /// Total number of hosts targeted by this deployment.
    #[serde(rename = "total_hosts")]
    pub total_hosts: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FleetDeploymentAttributes {
    pub fn new() -> FleetDeploymentAttributes {
        FleetDeploymentAttributes {
            config_operations: None,
            estimated_end_time_unix: None,
            filter_query: None,
            high_level_status: None,
            total_hosts: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn config_operations(
        mut self,
        value: Vec<crate::datadogV2::model::FleetDeploymentOperation>,
    ) -> Self {
        self.config_operations = Some(value);
        self
    }

    pub fn estimated_end_time_unix(mut self, value: i64) -> Self {
        self.estimated_end_time_unix = Some(value);
        self
    }

    pub fn filter_query(mut self, value: String) -> Self {
        self.filter_query = Some(value);
        self
    }

    pub fn high_level_status(mut self, value: String) -> Self {
        self.high_level_status = Some(value);
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

impl Default for FleetDeploymentAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for FleetDeploymentAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FleetDeploymentAttributesVisitor;
        impl<'a> Visitor<'a> for FleetDeploymentAttributesVisitor {
            type Value = FleetDeploymentAttributes;

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
                let mut estimated_end_time_unix: Option<i64> = None;
                let mut filter_query: Option<String> = None;
                let mut high_level_status: Option<String> = None;
                let mut total_hosts: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "config_operations" => {
                            if v.is_null() {
                                continue;
                            }
                            config_operations =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "estimated_end_time_unix" => {
                            if v.is_null() {
                                continue;
                            }
                            estimated_end_time_unix =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "filter_query" => {
                            if v.is_null() {
                                continue;
                            }
                            filter_query =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "high_level_status" => {
                            if v.is_null() {
                                continue;
                            }
                            high_level_status =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = FleetDeploymentAttributes {
                    config_operations,
                    estimated_end_time_unix,
                    filter_query,
                    high_level_status,
                    total_hosts,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FleetDeploymentAttributesVisitor)
    }
}

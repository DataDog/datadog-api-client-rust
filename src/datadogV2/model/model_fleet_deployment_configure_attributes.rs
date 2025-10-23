// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for creating a new configuration deployment.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FleetDeploymentConfigureAttributes {
    /// Ordered list of configuration file operations to perform on the target hosts.
    #[serde(rename = "config_operations")]
    pub config_operations: Vec<crate::datadogV2::model::FleetDeploymentOperation>,
    /// Query used to filter and select target hosts for the deployment. Uses the Datadog query syntax.
    #[serde(rename = "filter_query")]
    pub filter_query: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FleetDeploymentConfigureAttributes {
    pub fn new(
        config_operations: Vec<crate::datadogV2::model::FleetDeploymentOperation>,
    ) -> FleetDeploymentConfigureAttributes {
        FleetDeploymentConfigureAttributes {
            config_operations,
            filter_query: None,
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

impl<'de> Deserialize<'de> for FleetDeploymentConfigureAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FleetDeploymentConfigureAttributesVisitor;
        impl<'a> Visitor<'a> for FleetDeploymentConfigureAttributesVisitor {
            type Value = FleetDeploymentConfigureAttributes;

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
                let mut filter_query: Option<String> = None;
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
                        "filter_query" => {
                            if v.is_null() {
                                continue;
                            }
                            filter_query =
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

                let content = FleetDeploymentConfigureAttributes {
                    config_operations,
                    filter_query,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FleetDeploymentConfigureAttributesVisitor)
    }
}

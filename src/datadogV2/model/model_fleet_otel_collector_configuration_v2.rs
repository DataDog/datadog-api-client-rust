// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Configuration for a single OpenTelemetry collector associated with the agent.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FleetOtelCollectorConfigurationV2 {
    /// The unique identifier of the OpenTelemetry collector.
    #[serde(rename = "collector_id")]
    pub collector_id: Option<String>,
    /// The final compiled configuration of the OpenTelemetry collector.
    #[serde(rename = "compiled_configuration")]
    pub compiled_configuration: Option<String>,
    /// The distribution of the OpenTelemetry collector.
    #[serde(rename = "distribution")]
    pub distribution: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FleetOtelCollectorConfigurationV2 {
    pub fn new() -> FleetOtelCollectorConfigurationV2 {
        FleetOtelCollectorConfigurationV2 {
            collector_id: None,
            compiled_configuration: None,
            distribution: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn collector_id(mut self, value: String) -> Self {
        self.collector_id = Some(value);
        self
    }

    pub fn compiled_configuration(mut self, value: String) -> Self {
        self.compiled_configuration = Some(value);
        self
    }

    pub fn distribution(mut self, value: String) -> Self {
        self.distribution = Some(value);
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

impl Default for FleetOtelCollectorConfigurationV2 {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for FleetOtelCollectorConfigurationV2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FleetOtelCollectorConfigurationV2Visitor;
        impl<'a> Visitor<'a> for FleetOtelCollectorConfigurationV2Visitor {
            type Value = FleetOtelCollectorConfigurationV2;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut collector_id: Option<String> = None;
                let mut compiled_configuration: Option<String> = None;
                let mut distribution: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "collector_id" => {
                            if v.is_null() {
                                continue;
                            }
                            collector_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "compiled_configuration" => {
                            if v.is_null() {
                                continue;
                            }
                            compiled_configuration =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "distribution" => {
                            if v.is_null() {
                                continue;
                            }
                            distribution =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = FleetOtelCollectorConfigurationV2 {
                    collector_id,
                    compiled_configuration,
                    distribution,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FleetOtelCollectorConfigurationV2Visitor)
    }
}

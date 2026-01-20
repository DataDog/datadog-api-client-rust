// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Defines how the `datadog_logs` destination routes matching logs to a Datadog site using a specific API key.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineDatadogLogsDestinationRoute {
    /// Name of the environment variable or secret that stores the Datadog API key used by this route.
    #[serde(rename = "api_key_key")]
    pub api_key_key: Option<String>,
    /// A Datadog search query that determines which logs are forwarded using this route.
    #[serde(rename = "include")]
    pub include: Option<String>,
    /// Unique identifier for this route within the destination.
    #[serde(rename = "route_id")]
    pub route_id: Option<String>,
    /// Datadog site where matching logs are sent (for example, `us1`).
    #[serde(rename = "site")]
    pub site: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineDatadogLogsDestinationRoute {
    pub fn new() -> ObservabilityPipelineDatadogLogsDestinationRoute {
        ObservabilityPipelineDatadogLogsDestinationRoute {
            api_key_key: None,
            include: None,
            route_id: None,
            site: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn api_key_key(mut self, value: String) -> Self {
        self.api_key_key = Some(value);
        self
    }

    pub fn include(mut self, value: String) -> Self {
        self.include = Some(value);
        self
    }

    pub fn route_id(mut self, value: String) -> Self {
        self.route_id = Some(value);
        self
    }

    pub fn site(mut self, value: String) -> Self {
        self.site = Some(value);
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

impl Default for ObservabilityPipelineDatadogLogsDestinationRoute {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ObservabilityPipelineDatadogLogsDestinationRoute {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineDatadogLogsDestinationRouteVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineDatadogLogsDestinationRouteVisitor {
            type Value = ObservabilityPipelineDatadogLogsDestinationRoute;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut api_key_key: Option<String> = None;
                let mut include: Option<String> = None;
                let mut route_id: Option<String> = None;
                let mut site: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "api_key_key" => {
                            if v.is_null() {
                                continue;
                            }
                            api_key_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "include" => {
                            if v.is_null() {
                                continue;
                            }
                            include = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "route_id" => {
                            if v.is_null() {
                                continue;
                            }
                            route_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "site" => {
                            if v.is_null() {
                                continue;
                            }
                            site = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ObservabilityPipelineDatadogLogsDestinationRoute {
                    api_key_key,
                    include,
                    route_id,
                    site,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineDatadogLogsDestinationRouteVisitor)
    }
}

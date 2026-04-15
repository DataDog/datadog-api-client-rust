// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a fleet tracer representing a service instance reporting telemetry.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FleetTracerAttributes {
    /// The environment the tracer is reporting from.
    #[serde(rename = "env")]
    pub env: Option<String>,
    /// The hostname where the tracer is running.
    #[serde(rename = "hostname")]
    pub hostname: Option<String>,
    /// The programming language of the traced application.
    #[serde(rename = "language")]
    pub language: Option<String>,
    /// The version of the programming language runtime.
    #[serde(rename = "language_version")]
    pub language_version: Option<String>,
    /// The remote configuration status of the tracer.
    #[serde(rename = "remote_config_status")]
    pub remote_config_status: Option<String>,
    /// Runtime identifiers for the tracer instances.
    #[serde(rename = "runtime_ids")]
    pub runtime_ids: Option<Vec<String>>,
    /// The telemetry-derived service name reported by the tracer.
    #[serde(rename = "service")]
    pub service: Option<String>,
    /// The service hostname reported by the tracer.
    #[serde(rename = "service_hostname")]
    pub service_hostname: Option<String>,
    /// The version of the traced service.
    #[serde(rename = "service_version")]
    pub service_version: Option<String>,
    /// The version of the Datadog tracer library.
    #[serde(rename = "tracer_version")]
    pub tracer_version: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FleetTracerAttributes {
    pub fn new() -> FleetTracerAttributes {
        FleetTracerAttributes {
            env: None,
            hostname: None,
            language: None,
            language_version: None,
            remote_config_status: None,
            runtime_ids: None,
            service: None,
            service_hostname: None,
            service_version: None,
            tracer_version: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn env(mut self, value: String) -> Self {
        self.env = Some(value);
        self
    }

    pub fn hostname(mut self, value: String) -> Self {
        self.hostname = Some(value);
        self
    }

    pub fn language(mut self, value: String) -> Self {
        self.language = Some(value);
        self
    }

    pub fn language_version(mut self, value: String) -> Self {
        self.language_version = Some(value);
        self
    }

    pub fn remote_config_status(mut self, value: String) -> Self {
        self.remote_config_status = Some(value);
        self
    }

    pub fn runtime_ids(mut self, value: Vec<String>) -> Self {
        self.runtime_ids = Some(value);
        self
    }

    pub fn service(mut self, value: String) -> Self {
        self.service = Some(value);
        self
    }

    pub fn service_hostname(mut self, value: String) -> Self {
        self.service_hostname = Some(value);
        self
    }

    pub fn service_version(mut self, value: String) -> Self {
        self.service_version = Some(value);
        self
    }

    pub fn tracer_version(mut self, value: String) -> Self {
        self.tracer_version = Some(value);
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

impl Default for FleetTracerAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for FleetTracerAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FleetTracerAttributesVisitor;
        impl<'a> Visitor<'a> for FleetTracerAttributesVisitor {
            type Value = FleetTracerAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut env: Option<String> = None;
                let mut hostname: Option<String> = None;
                let mut language: Option<String> = None;
                let mut language_version: Option<String> = None;
                let mut remote_config_status: Option<String> = None;
                let mut runtime_ids: Option<Vec<String>> = None;
                let mut service: Option<String> = None;
                let mut service_hostname: Option<String> = None;
                let mut service_version: Option<String> = None;
                let mut tracer_version: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "env" => {
                            if v.is_null() {
                                continue;
                            }
                            env = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "hostname" => {
                            if v.is_null() {
                                continue;
                            }
                            hostname = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "language" => {
                            if v.is_null() {
                                continue;
                            }
                            language = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "language_version" => {
                            if v.is_null() {
                                continue;
                            }
                            language_version =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "remote_config_status" => {
                            if v.is_null() {
                                continue;
                            }
                            remote_config_status =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "runtime_ids" => {
                            if v.is_null() {
                                continue;
                            }
                            runtime_ids =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "service" => {
                            if v.is_null() {
                                continue;
                            }
                            service = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "service_hostname" => {
                            if v.is_null() {
                                continue;
                            }
                            service_hostname =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "service_version" => {
                            if v.is_null() {
                                continue;
                            }
                            service_version =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tracer_version" => {
                            if v.is_null() {
                                continue;
                            }
                            tracer_version =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = FleetTracerAttributes {
                    env,
                    hostname,
                    language,
                    language_version,
                    remote_config_status,
                    runtime_ids,
                    service,
                    service_hostname,
                    service_version,
                    tracer_version,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FleetTracerAttributesVisitor)
    }
}

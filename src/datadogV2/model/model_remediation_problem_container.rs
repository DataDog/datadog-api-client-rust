// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A container within a problematic task that is exhibiting issues.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RemediationProblemContainer {
    /// CPU limit.
    #[serde(rename = "cpu_limit")]
    pub cpu_limit: Option<i64>,
    /// Exit code if the container stopped.
    #[serde(rename = "exit_code")]
    pub exit_code: Option<i32>,
    /// Container health status.
    #[serde(rename = "health_status")]
    pub health_status: Option<String>,
    /// Docker image URI.
    #[serde(rename = "image")]
    pub image: Option<String>,
    /// Container status.
    #[serde(rename = "last_status")]
    pub last_status: Option<String>,
    /// Memory limit in MiB.
    #[serde(rename = "memory_limit_mib")]
    pub memory_limit_mib: Option<i64>,
    /// Container name from the task definition.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Stop reason.
    #[serde(rename = "reason")]
    pub reason: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RemediationProblemContainer {
    pub fn new() -> RemediationProblemContainer {
        RemediationProblemContainer {
            cpu_limit: None,
            exit_code: None,
            health_status: None,
            image: None,
            last_status: None,
            memory_limit_mib: None,
            name: None,
            reason: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn cpu_limit(mut self, value: i64) -> Self {
        self.cpu_limit = Some(value);
        self
    }

    pub fn exit_code(mut self, value: i32) -> Self {
        self.exit_code = Some(value);
        self
    }

    pub fn health_status(mut self, value: String) -> Self {
        self.health_status = Some(value);
        self
    }

    pub fn image(mut self, value: String) -> Self {
        self.image = Some(value);
        self
    }

    pub fn last_status(mut self, value: String) -> Self {
        self.last_status = Some(value);
        self
    }

    pub fn memory_limit_mib(mut self, value: i64) -> Self {
        self.memory_limit_mib = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn reason(mut self, value: String) -> Self {
        self.reason = Some(value);
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

impl Default for RemediationProblemContainer {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for RemediationProblemContainer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RemediationProblemContainerVisitor;
        impl<'a> Visitor<'a> for RemediationProblemContainerVisitor {
            type Value = RemediationProblemContainer;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut cpu_limit: Option<i64> = None;
                let mut exit_code: Option<i32> = None;
                let mut health_status: Option<String> = None;
                let mut image: Option<String> = None;
                let mut last_status: Option<String> = None;
                let mut memory_limit_mib: Option<i64> = None;
                let mut name: Option<String> = None;
                let mut reason: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "cpu_limit" => {
                            if v.is_null() {
                                continue;
                            }
                            cpu_limit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "exit_code" => {
                            if v.is_null() {
                                continue;
                            }
                            exit_code = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "health_status" => {
                            if v.is_null() {
                                continue;
                            }
                            health_status =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "image" => {
                            if v.is_null() {
                                continue;
                            }
                            image = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_status" => {
                            if v.is_null() {
                                continue;
                            }
                            last_status =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "memory_limit_mib" => {
                            if v.is_null() {
                                continue;
                            }
                            memory_limit_mib =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "reason" => {
                            if v.is_null() {
                                continue;
                            }
                            reason = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = RemediationProblemContainer {
                    cpu_limit,
                    exit_code,
                    health_status,
                    image,
                    last_status,
                    memory_limit_mib,
                    name,
                    reason,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RemediationProblemContainerVisitor)
    }
}

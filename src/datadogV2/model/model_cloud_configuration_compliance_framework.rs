// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Compliance framework requirement a cloud_configuration rule maps to.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CloudConfigurationComplianceFramework {
    /// Control of the compliance framework requirement.
    #[serde(rename = "control")]
    pub control: Option<String>,
    /// Handle of the compliance framework.
    #[serde(rename = "framework")]
    pub framework: Option<String>,
    /// Whether the mapping comes from the default rule rather than from a custom framework.
    #[serde(rename = "is_default")]
    pub is_default: Option<bool>,
    /// Message describing how the rule maps to the compliance framework requirement.
    #[serde(rename = "message")]
    pub message: Option<String>,
    /// Requirement of the compliance framework.
    #[serde(rename = "requirement")]
    pub requirement: Option<String>,
    /// Version of the compliance framework.
    #[serde(rename = "version")]
    pub version: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CloudConfigurationComplianceFramework {
    pub fn new() -> CloudConfigurationComplianceFramework {
        CloudConfigurationComplianceFramework {
            control: None,
            framework: None,
            is_default: None,
            message: None,
            requirement: None,
            version: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn control(mut self, value: String) -> Self {
        self.control = Some(value);
        self
    }

    pub fn framework(mut self, value: String) -> Self {
        self.framework = Some(value);
        self
    }

    pub fn is_default(mut self, value: bool) -> Self {
        self.is_default = Some(value);
        self
    }

    pub fn message(mut self, value: String) -> Self {
        self.message = Some(value);
        self
    }

    pub fn requirement(mut self, value: String) -> Self {
        self.requirement = Some(value);
        self
    }

    pub fn version(mut self, value: String) -> Self {
        self.version = Some(value);
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

impl Default for CloudConfigurationComplianceFramework {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CloudConfigurationComplianceFramework {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CloudConfigurationComplianceFrameworkVisitor;
        impl<'a> Visitor<'a> for CloudConfigurationComplianceFrameworkVisitor {
            type Value = CloudConfigurationComplianceFramework;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut control: Option<String> = None;
                let mut framework: Option<String> = None;
                let mut is_default: Option<bool> = None;
                let mut message: Option<String> = None;
                let mut requirement: Option<String> = None;
                let mut version: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "control" => {
                            if v.is_null() {
                                continue;
                            }
                            control = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "framework" => {
                            if v.is_null() {
                                continue;
                            }
                            framework = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_default" => {
                            if v.is_null() {
                                continue;
                            }
                            is_default = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "message" => {
                            if v.is_null() {
                                continue;
                            }
                            message = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "requirement" => {
                            if v.is_null() {
                                continue;
                            }
                            requirement =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "version" => {
                            if v.is_null() {
                                continue;
                            }
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = CloudConfigurationComplianceFramework {
                    control,
                    framework,
                    is_default,
                    message,
                    requirement,
                    version,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CloudConfigurationComplianceFrameworkVisitor)
    }
}

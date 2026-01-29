// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Monitor integration settings
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IntegrationMonitor {
    /// Whether auto-resolve is enabled
    #[serde(rename = "auto_resolve_enabled")]
    pub auto_resolve_enabled: Option<bool>,
    /// Case type ID for monitor integration
    #[serde(rename = "case_type_id")]
    pub case_type_id: Option<String>,
    /// Whether monitor integration is enabled
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// Monitor handle
    #[serde(rename = "handle")]
    pub handle: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IntegrationMonitor {
    pub fn new() -> IntegrationMonitor {
        IntegrationMonitor {
            auto_resolve_enabled: None,
            case_type_id: None,
            enabled: None,
            handle: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn auto_resolve_enabled(mut self, value: bool) -> Self {
        self.auto_resolve_enabled = Some(value);
        self
    }

    pub fn case_type_id(mut self, value: String) -> Self {
        self.case_type_id = Some(value);
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn handle(mut self, value: String) -> Self {
        self.handle = Some(value);
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

impl Default for IntegrationMonitor {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for IntegrationMonitor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IntegrationMonitorVisitor;
        impl<'a> Visitor<'a> for IntegrationMonitorVisitor {
            type Value = IntegrationMonitor;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut auto_resolve_enabled: Option<bool> = None;
                let mut case_type_id: Option<String> = None;
                let mut enabled: Option<bool> = None;
                let mut handle: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "auto_resolve_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            auto_resolve_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "case_type_id" => {
                            if v.is_null() {
                                continue;
                            }
                            case_type_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "handle" => {
                            if v.is_null() {
                                continue;
                            }
                            handle = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = IntegrationMonitor {
                    auto_resolve_enabled,
                    case_type_id,
                    enabled,
                    handle,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IntegrationMonitorVisitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// ServiceNow integration settings
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IntegrationServiceNow {
    /// Assignment group
    #[serde(rename = "assignment_group")]
    pub assignment_group: Option<String>,
    #[serde(rename = "auto_creation")]
    pub auto_creation: Option<crate::datadogV2::model::IntegrationServiceNowAutoCreation>,
    /// Whether ServiceNow integration is enabled
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// ServiceNow instance name
    #[serde(rename = "instance_name")]
    pub instance_name: Option<String>,
    #[serde(rename = "sync_config")]
    pub sync_config: Option<crate::datadogV2::model::IntegrationServiceNowSyncConfig>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IntegrationServiceNow {
    pub fn new() -> IntegrationServiceNow {
        IntegrationServiceNow {
            assignment_group: None,
            auto_creation: None,
            enabled: None,
            instance_name: None,
            sync_config: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn assignment_group(mut self, value: String) -> Self {
        self.assignment_group = Some(value);
        self
    }

    pub fn auto_creation(
        mut self,
        value: crate::datadogV2::model::IntegrationServiceNowAutoCreation,
    ) -> Self {
        self.auto_creation = Some(value);
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn instance_name(mut self, value: String) -> Self {
        self.instance_name = Some(value);
        self
    }

    pub fn sync_config(
        mut self,
        value: crate::datadogV2::model::IntegrationServiceNowSyncConfig,
    ) -> Self {
        self.sync_config = Some(value);
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

impl Default for IntegrationServiceNow {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for IntegrationServiceNow {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IntegrationServiceNowVisitor;
        impl<'a> Visitor<'a> for IntegrationServiceNowVisitor {
            type Value = IntegrationServiceNow;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut assignment_group: Option<String> = None;
                let mut auto_creation: Option<
                    crate::datadogV2::model::IntegrationServiceNowAutoCreation,
                > = None;
                let mut enabled: Option<bool> = None;
                let mut instance_name: Option<String> = None;
                let mut sync_config: Option<
                    crate::datadogV2::model::IntegrationServiceNowSyncConfig,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "assignment_group" => {
                            if v.is_null() {
                                continue;
                            }
                            assignment_group =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "auto_creation" => {
                            if v.is_null() {
                                continue;
                            }
                            auto_creation =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "instance_name" => {
                            if v.is_null() {
                                continue;
                            }
                            instance_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sync_config" => {
                            if v.is_null() {
                                continue;
                            }
                            sync_config =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = IntegrationServiceNow {
                    assignment_group,
                    auto_creation,
                    enabled,
                    instance_name,
                    sync_config,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IntegrationServiceNowVisitor)
    }
}

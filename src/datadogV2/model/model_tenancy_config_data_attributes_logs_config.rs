// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TenancyConfigDataAttributesLogsConfig {
    #[serde(rename = "compartment_tag_filters")]
    pub compartment_tag_filters: Option<Vec<String>>,
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    #[serde(rename = "enabled_services")]
    pub enabled_services: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TenancyConfigDataAttributesLogsConfig {
    pub fn new() -> TenancyConfigDataAttributesLogsConfig {
        TenancyConfigDataAttributesLogsConfig {
            compartment_tag_filters: None,
            enabled: None,
            enabled_services: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn compartment_tag_filters(mut self, value: Vec<String>) -> Self {
        self.compartment_tag_filters = Some(value);
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn enabled_services(mut self, value: Vec<String>) -> Self {
        self.enabled_services = Some(value);
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

impl Default for TenancyConfigDataAttributesLogsConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TenancyConfigDataAttributesLogsConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TenancyConfigDataAttributesLogsConfigVisitor;
        impl<'a> Visitor<'a> for TenancyConfigDataAttributesLogsConfigVisitor {
            type Value = TenancyConfigDataAttributesLogsConfig;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut compartment_tag_filters: Option<Vec<String>> = None;
                let mut enabled: Option<bool> = None;
                let mut enabled_services: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "compartment_tag_filters" => {
                            if v.is_null() {
                                continue;
                            }
                            compartment_tag_filters =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enabled_services" => {
                            if v.is_null() {
                                continue;
                            }
                            enabled_services =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = TenancyConfigDataAttributesLogsConfig {
                    compartment_tag_filters,
                    enabled,
                    enabled_services,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TenancyConfigDataAttributesLogsConfigVisitor)
    }
}

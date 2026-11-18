// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Fleet actions authorized on agents matching the access scope query.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FleetAccessScopeFeatures {
    /// Whether the scope permits configuration changes.
    #[serde(rename = "configure")]
    pub configure: Option<bool>,
    /// Whether the scope permits Agent installation.
    #[serde(rename = "install")]
    pub install: Option<bool>,
    /// Whether the scope permits package upgrades.
    #[serde(rename = "upgrade")]
    pub upgrade: Option<bool>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FleetAccessScopeFeatures {
    pub fn new() -> FleetAccessScopeFeatures {
        FleetAccessScopeFeatures {
            configure: None,
            install: None,
            upgrade: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn configure(mut self, value: bool) -> Self {
        self.configure = Some(value);
        self
    }

    pub fn install(mut self, value: bool) -> Self {
        self.install = Some(value);
        self
    }

    pub fn upgrade(mut self, value: bool) -> Self {
        self.upgrade = Some(value);
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

impl Default for FleetAccessScopeFeatures {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for FleetAccessScopeFeatures {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FleetAccessScopeFeaturesVisitor;
        impl<'a> Visitor<'a> for FleetAccessScopeFeaturesVisitor {
            type Value = FleetAccessScopeFeatures;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut configure: Option<bool> = None;
                let mut install: Option<bool> = None;
                let mut upgrade: Option<bool> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "configure" => {
                            if v.is_null() {
                                continue;
                            }
                            configure = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "install" => {
                            if v.is_null() {
                                continue;
                            }
                            install = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "upgrade" => {
                            if v.is_null() {
                                continue;
                            }
                            upgrade = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = FleetAccessScopeFeatures {
                    configure,
                    install,
                    upgrade,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FleetAccessScopeFeaturesVisitor)
    }
}

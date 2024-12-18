// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Configuration settings applied to resources from the specified Azure resource provider.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ResourceProviderConfig {
    /// Collect metrics for resources from this provider.
    #[serde(rename = "metrics_enabled")]
    pub metrics_enabled: Option<bool>,
    /// The provider namespace to apply this configuration to.
    #[serde(rename = "namespace")]
    pub namespace: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ResourceProviderConfig {
    pub fn new() -> ResourceProviderConfig {
        ResourceProviderConfig {
            metrics_enabled: None,
            namespace: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn metrics_enabled(mut self, value: bool) -> Self {
        self.metrics_enabled = Some(value);
        self
    }

    pub fn namespace(mut self, value: String) -> Self {
        self.namespace = Some(value);
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

impl Default for ResourceProviderConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ResourceProviderConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ResourceProviderConfigVisitor;
        impl<'a> Visitor<'a> for ResourceProviderConfigVisitor {
            type Value = ResourceProviderConfig;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut metrics_enabled: Option<bool> = None;
                let mut namespace: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "metrics_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            metrics_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "namespace" => {
                            if v.is_null() {
                                continue;
                            }
                            namespace = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ResourceProviderConfig {
                    metrics_enabled,
                    namespace,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ResourceProviderConfigVisitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for requesting Test Optimization service settings.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TestOptimizationGetServiceSettingsRequestAttributes {
    /// The environment name. If omitted, defaults to `none`.
    #[serde(rename = "env")]
    pub env: Option<String>,
    /// The repository identifier.
    #[serde(rename = "repository_id")]
    pub repository_id: String,
    /// The service name.
    #[serde(rename = "service_name")]
    pub service_name: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TestOptimizationGetServiceSettingsRequestAttributes {
    pub fn new(
        repository_id: String,
        service_name: String,
    ) -> TestOptimizationGetServiceSettingsRequestAttributes {
        TestOptimizationGetServiceSettingsRequestAttributes {
            env: None,
            repository_id,
            service_name,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn env(mut self, value: String) -> Self {
        self.env = Some(value);
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

impl<'de> Deserialize<'de> for TestOptimizationGetServiceSettingsRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TestOptimizationGetServiceSettingsRequestAttributesVisitor;
        impl<'a> Visitor<'a> for TestOptimizationGetServiceSettingsRequestAttributesVisitor {
            type Value = TestOptimizationGetServiceSettingsRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut env: Option<String> = None;
                let mut repository_id: Option<String> = None;
                let mut service_name: Option<String> = None;
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
                        "repository_id" => {
                            repository_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "service_name" => {
                            service_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let repository_id =
                    repository_id.ok_or_else(|| M::Error::missing_field("repository_id"))?;
                let service_name =
                    service_name.ok_or_else(|| M::Error::missing_field("service_name"))?;

                let content = TestOptimizationGetServiceSettingsRequestAttributes {
                    env,
                    repository_id,
                    service_name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TestOptimizationGetServiceSettingsRequestAttributesVisitor)
    }
}

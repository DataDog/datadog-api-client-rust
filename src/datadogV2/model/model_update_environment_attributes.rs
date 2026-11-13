// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for updating an environment.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UpdateEnvironmentAttributes {
    /// Indicates whether this is a production environment. If omitted, resets to false. Include the existing value to preserve it.
    #[serde(rename = "is_production")]
    pub is_production: Option<bool>,
    /// The name of the environment. Required on every request and unique within the organization. Must contain 1 to 200 ASCII characters, start with a letter or digit, and contain only letters, digits, spaces, underscores, hyphens, and periods.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Controls whether feature flag evaluation events may include original targeting identifiers and full evaluation context. When true, SDKs may include this data. When false, SDKs hash targeting identifiers and omit evaluation context. If omitted, the existing value is preserved.
    #[serde(rename = "observe_full_evaluation_data")]
    pub observe_full_evaluation_data: Option<bool>,
    /// Required list of at least one exact DD_ENV value associated with this environment. Values cannot be used by another environment in the same organization. Wildcards and search expressions are not supported.
    #[serde(rename = "queries")]
    pub queries: Option<Vec<String>>,
    /// Indicates whether feature flag changes require approval in this environment. If omitted, resets to false. Include the existing value to preserve it.
    #[serde(rename = "require_feature_flag_approval")]
    pub require_feature_flag_approval: Option<bool>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UpdateEnvironmentAttributes {
    pub fn new() -> UpdateEnvironmentAttributes {
        UpdateEnvironmentAttributes {
            is_production: None,
            name: None,
            observe_full_evaluation_data: None,
            queries: None,
            require_feature_flag_approval: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn is_production(mut self, value: bool) -> Self {
        self.is_production = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn observe_full_evaluation_data(mut self, value: bool) -> Self {
        self.observe_full_evaluation_data = Some(value);
        self
    }

    pub fn queries(mut self, value: Vec<String>) -> Self {
        self.queries = Some(value);
        self
    }

    pub fn require_feature_flag_approval(mut self, value: bool) -> Self {
        self.require_feature_flag_approval = Some(value);
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

impl Default for UpdateEnvironmentAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for UpdateEnvironmentAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UpdateEnvironmentAttributesVisitor;
        impl<'a> Visitor<'a> for UpdateEnvironmentAttributesVisitor {
            type Value = UpdateEnvironmentAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut is_production: Option<bool> = None;
                let mut name: Option<String> = None;
                let mut observe_full_evaluation_data: Option<bool> = None;
                let mut queries: Option<Vec<String>> = None;
                let mut require_feature_flag_approval: Option<bool> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "is_production" => {
                            if v.is_null() {
                                continue;
                            }
                            is_production =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "observe_full_evaluation_data" => {
                            if v.is_null() {
                                continue;
                            }
                            observe_full_evaluation_data =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "queries" => {
                            if v.is_null() {
                                continue;
                            }
                            queries = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "require_feature_flag_approval" => {
                            if v.is_null() {
                                continue;
                            }
                            require_feature_flag_approval =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = UpdateEnvironmentAttributes {
                    is_production,
                    name,
                    observe_full_evaluation_data,
                    queries,
                    require_feature_flag_approval,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UpdateEnvironmentAttributesVisitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for creating a new environment.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CreateEnvironmentAttributes {
    /// Indicates whether this is a production environment.
    #[serde(rename = "is_production")]
    pub is_production: Option<bool>,
    /// The name of the environment.
    #[serde(rename = "name")]
    pub name: String,
    /// List of queries to define the environment scope.
    #[serde(rename = "queries")]
    pub queries: Vec<String>,
    /// Indicates whether feature flag changes require approval in this environment.
    #[serde(rename = "require_feature_flag_approval")]
    pub require_feature_flag_approval: Option<bool>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CreateEnvironmentAttributes {
    pub fn new(name: String, queries: Vec<String>) -> CreateEnvironmentAttributes {
        CreateEnvironmentAttributes {
            is_production: None,
            name,
            queries,
            require_feature_flag_approval: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn is_production(mut self, value: bool) -> Self {
        self.is_production = Some(value);
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

impl<'de> Deserialize<'de> for CreateEnvironmentAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CreateEnvironmentAttributesVisitor;
        impl<'a> Visitor<'a> for CreateEnvironmentAttributesVisitor {
            type Value = CreateEnvironmentAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut is_production: Option<bool> = None;
                let mut name: Option<String> = None;
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
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "queries" => {
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
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let queries = queries.ok_or_else(|| M::Error::missing_field("queries"))?;

                let content = CreateEnvironmentAttributes {
                    is_production,
                    name,
                    queries,
                    require_feature_flag_approval,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CreateEnvironmentAttributesVisitor)
    }
}

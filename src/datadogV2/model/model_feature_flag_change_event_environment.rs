// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The environment the change applies to. Null for a change to the flag itself rather than to one environment.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FeatureFlagChangeEventEnvironment {
    /// The ID of the environment.
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    /// Whether the environment is a production environment.
    #[serde(rename = "is_production")]
    pub is_production: bool,
    /// The name of the environment.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FeatureFlagChangeEventEnvironment {
    pub fn new(
        id: uuid::Uuid,
        is_production: bool,
        name: String,
    ) -> FeatureFlagChangeEventEnvironment {
        FeatureFlagChangeEventEnvironment {
            id,
            is_production,
            name,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for FeatureFlagChangeEventEnvironment {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FeatureFlagChangeEventEnvironmentVisitor;
        impl<'a> Visitor<'a> for FeatureFlagChangeEventEnvironmentVisitor {
            type Value = FeatureFlagChangeEventEnvironment;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut id: Option<uuid::Uuid> = None;
                let mut is_production: Option<bool> = None;
                let mut name: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_production" => {
                            is_production =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let is_production =
                    is_production.ok_or_else(|| M::Error::missing_field("is_production"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = FeatureFlagChangeEventEnvironment {
                    id,
                    is_production,
                    name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FeatureFlagChangeEventEnvironmentVisitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of Entity V3 Repository Spec object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct EntityV3RepositorySpec {
    /// A list of components the repository is a part of.
    #[serde(rename = "componentOf")]
    pub component_of: Option<Vec<String>>,
    /// A list of components that the repository is a dependency of.
    #[serde(rename = "dependencyOf")]
    pub dependency_of: Option<Vec<String>>,
    /// A list of components that the repository depends on.
    #[serde(rename = "dependsOn")]
    pub depends_on: Option<Vec<String>>,
    /// The lifecycle state of the repository.
    #[serde(rename = "lifecycle")]
    pub lifecycle: Option<String>,
    /// The importance of the repository.
    #[serde(rename = "tier")]
    pub tier: Option<String>,
    /// The type of repository.
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl EntityV3RepositorySpec {
    pub fn new() -> EntityV3RepositorySpec {
        EntityV3RepositorySpec {
            component_of: None,
            dependency_of: None,
            depends_on: None,
            lifecycle: None,
            tier: None,
            type_: None,
            _unparsed: false,
        }
    }

    pub fn component_of(mut self, value: Vec<String>) -> Self {
        self.component_of = Some(value);
        self
    }

    pub fn dependency_of(mut self, value: Vec<String>) -> Self {
        self.dependency_of = Some(value);
        self
    }

    pub fn depends_on(mut self, value: Vec<String>) -> Self {
        self.depends_on = Some(value);
        self
    }

    pub fn lifecycle(mut self, value: String) -> Self {
        self.lifecycle = Some(value);
        self
    }

    pub fn tier(mut self, value: String) -> Self {
        self.tier = Some(value);
        self
    }

    pub fn type_(mut self, value: String) -> Self {
        self.type_ = Some(value);
        self
    }
}

impl Default for EntityV3RepositorySpec {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for EntityV3RepositorySpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EntityV3RepositorySpecVisitor;
        impl<'a> Visitor<'a> for EntityV3RepositorySpecVisitor {
            type Value = EntityV3RepositorySpec;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut component_of: Option<Vec<String>> = None;
                let mut dependency_of: Option<Vec<String>> = None;
                let mut depends_on: Option<Vec<String>> = None;
                let mut lifecycle: Option<String> = None;
                let mut tier: Option<String> = None;
                let mut type_: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "componentOf" => {
                            if v.is_null() {
                                continue;
                            }
                            component_of =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "dependencyOf" => {
                            if v.is_null() {
                                continue;
                            }
                            dependency_of =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "dependsOn" => {
                            if v.is_null() {
                                continue;
                            }
                            depends_on = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "lifecycle" => {
                            if v.is_null() {
                                continue;
                            }
                            lifecycle = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tier" => {
                            if v.is_null() {
                                continue;
                            }
                            tier = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }

                let content = EntityV3RepositorySpec {
                    component_of,
                    dependency_of,
                    depends_on,
                    lifecycle,
                    tier,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(EntityV3RepositorySpecVisitor)
    }
}

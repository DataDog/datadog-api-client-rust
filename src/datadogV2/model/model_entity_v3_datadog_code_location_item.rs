// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Code location item.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct EntityV3DatadogCodeLocationItem {
    /// The paths (glob) to the source code of the service.
    #[serde(rename = "paths")]
    pub paths: Option<Vec<String>>,
    /// The repository path of the source code of the entity.
    #[serde(rename = "repositoryURL")]
    pub repository_url: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl EntityV3DatadogCodeLocationItem {
    pub fn new() -> EntityV3DatadogCodeLocationItem {
        EntityV3DatadogCodeLocationItem {
            paths: None,
            repository_url: None,
            _unparsed: false,
        }
    }

    pub fn paths(mut self, value: Vec<String>) -> Self {
        self.paths = Some(value);
        self
    }

    pub fn repository_url(mut self, value: String) -> Self {
        self.repository_url = Some(value);
        self
    }
}

impl Default for EntityV3DatadogCodeLocationItem {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for EntityV3DatadogCodeLocationItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EntityV3DatadogCodeLocationItemVisitor;
        impl<'a> Visitor<'a> for EntityV3DatadogCodeLocationItemVisitor {
            type Value = EntityV3DatadogCodeLocationItem;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut paths: Option<Vec<String>> = None;
                let mut repository_url: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "paths" => {
                            if v.is_null() {
                                continue;
                            }
                            paths = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "repositoryURL" => {
                            if v.is_null() {
                                continue;
                            }
                            repository_url =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }

                let content = EntityV3DatadogCodeLocationItem {
                    paths,
                    repository_url,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(EntityV3DatadogCodeLocationItemVisitor)
    }
}

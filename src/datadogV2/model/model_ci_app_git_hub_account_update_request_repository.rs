// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Repository-level opt-in change to apply, identified by name.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CIAppGitHubAccountUpdateRequestRepository {
    /// Whether to enable or disable CI Visibility for this repository.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// The repository name to update.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CIAppGitHubAccountUpdateRequestRepository {
    pub fn new(enabled: bool, name: String) -> CIAppGitHubAccountUpdateRequestRepository {
        CIAppGitHubAccountUpdateRequestRepository {
            enabled,
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

impl<'de> Deserialize<'de> for CIAppGitHubAccountUpdateRequestRepository {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CIAppGitHubAccountUpdateRequestRepositoryVisitor;
        impl<'a> Visitor<'a> for CIAppGitHubAccountUpdateRequestRepositoryVisitor {
            type Value = CIAppGitHubAccountUpdateRequestRepository;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut enabled: Option<bool> = None;
                let mut name: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "enabled" => {
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let enabled = enabled.ok_or_else(|| M::Error::missing_field("enabled"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = CIAppGitHubAccountUpdateRequestRepository {
                    enabled,
                    name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CIAppGitHubAccountUpdateRequestRepositoryVisitor)
    }
}

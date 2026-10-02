// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A repository and its files that reference the feature flag.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FeatureFlagStalenessCodeReference {
    /// Paths of files that reference the feature flag in this repository.
    #[serde(rename = "files")]
    pub files: Option<Vec<String>>,
    /// The URL of the source code repository, when available.
    #[serde(rename = "repo_url")]
    pub repo_url: Option<String>,
    /// The identifier of the source code repository.
    #[serde(rename = "scm_repository_id")]
    pub scm_repository_id: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FeatureFlagStalenessCodeReference {
    pub fn new() -> FeatureFlagStalenessCodeReference {
        FeatureFlagStalenessCodeReference {
            files: None,
            repo_url: None,
            scm_repository_id: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn files(mut self, value: Vec<String>) -> Self {
        self.files = Some(value);
        self
    }

    pub fn repo_url(mut self, value: String) -> Self {
        self.repo_url = Some(value);
        self
    }

    pub fn scm_repository_id(mut self, value: String) -> Self {
        self.scm_repository_id = Some(value);
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

impl Default for FeatureFlagStalenessCodeReference {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for FeatureFlagStalenessCodeReference {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FeatureFlagStalenessCodeReferenceVisitor;
        impl<'a> Visitor<'a> for FeatureFlagStalenessCodeReferenceVisitor {
            type Value = FeatureFlagStalenessCodeReference;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut files: Option<Vec<String>> = None;
                let mut repo_url: Option<String> = None;
                let mut scm_repository_id: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "files" => {
                            if v.is_null() {
                                continue;
                            }
                            files = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "repo_url" => {
                            if v.is_null() {
                                continue;
                            }
                            repo_url = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "scm_repository_id" => {
                            if v.is_null() {
                                continue;
                            }
                            scm_repository_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = FeatureFlagStalenessCodeReference {
                    files,
                    repo_url,
                    scm_repository_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FeatureFlagStalenessCodeReferenceVisitor)
    }
}

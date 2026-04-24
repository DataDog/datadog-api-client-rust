// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Git information associated with the test result.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTestResultGit {
    /// Git branch name.
    #[serde(rename = "branch")]
    pub branch: Option<String>,
    /// Details of the Git commit associated with the test result.
    #[serde(rename = "commit")]
    pub commit: Option<crate::datadogV2::model::SyntheticsTestResultGitCommit>,
    /// Git repository URL.
    #[serde(rename = "repository_url")]
    pub repository_url: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTestResultGit {
    pub fn new() -> SyntheticsTestResultGit {
        SyntheticsTestResultGit {
            branch: None,
            commit: None,
            repository_url: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn branch(mut self, value: String) -> Self {
        self.branch = Some(value);
        self
    }

    pub fn commit(mut self, value: crate::datadogV2::model::SyntheticsTestResultGitCommit) -> Self {
        self.commit = Some(value);
        self
    }

    pub fn repository_url(mut self, value: String) -> Self {
        self.repository_url = Some(value);
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

impl Default for SyntheticsTestResultGit {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsTestResultGit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTestResultGitVisitor;
        impl<'a> Visitor<'a> for SyntheticsTestResultGitVisitor {
            type Value = SyntheticsTestResultGit;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut branch: Option<String> = None;
                let mut commit: Option<crate::datadogV2::model::SyntheticsTestResultGitCommit> =
                    None;
                let mut repository_url: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "branch" => {
                            if v.is_null() {
                                continue;
                            }
                            branch = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "commit" => {
                            if v.is_null() {
                                continue;
                            }
                            commit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "repository_url" => {
                            if v.is_null() {
                                continue;
                            }
                            repository_url =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsTestResultGit {
                    branch,
                    commit,
                    repository_url,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTestResultGitVisitor)
    }
}

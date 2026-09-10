// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for requesting per-file code coverage data. Exactly one of `commit_sha`, `branch`, or `pr_number` must be provided. At most one of `service`, `codeowner`, or `flag` may be provided.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FilesCoverageRequestAttributes {
    /// The branch name.
    #[serde(rename = "branch")]
    pub branch: Option<String>,
    /// When true, return coverage data only for files that were changed in the specified scope.
    #[serde(rename = "changed_only")]
    pub changed_only: Option<bool>,
    /// Filter coverage by code owner. At most one of `service`, `codeowner`, or `flag` may be provided.
    #[serde(rename = "codeowner")]
    pub codeowner: Option<String>,
    /// The commit SHA (40-character hexadecimal string).
    #[serde(rename = "commit_sha")]
    pub commit_sha: Option<String>,
    /// Filter coverage by coverage flag. At most one of `service`, `codeowner`, or `flag` may be provided.
    #[serde(rename = "flag")]
    pub flag: Option<String>,
    /// The pull request number. Must be a positive integer.
    #[serde(rename = "pr_number")]
    pub pr_number: Option<i64>,
    /// Deprecated: use `repository_url` instead. The repository URL.
    #[deprecated]
    #[serde(rename = "repository_id")]
    pub repository_id: Option<String>,
    /// The repository URL. Accepts a full URL with or without a scheme (for example, `<https://github.com/org/repo`> or `github.com/org/repo`).
    #[serde(rename = "repository_url")]
    pub repository_url: Option<String>,
    /// Filter coverage by service name. At most one of `service`, `codeowner`, or `flag` may be provided.
    #[serde(rename = "service")]
    pub service: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FilesCoverageRequestAttributes {
    pub fn new() -> FilesCoverageRequestAttributes {
        #[allow(deprecated)]
        FilesCoverageRequestAttributes {
            branch: None,
            changed_only: None,
            codeowner: None,
            commit_sha: None,
            flag: None,
            pr_number: None,
            repository_id: None,
            repository_url: None,
            service: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    #[allow(deprecated)]
    pub fn branch(mut self, value: String) -> Self {
        self.branch = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn changed_only(mut self, value: bool) -> Self {
        self.changed_only = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn codeowner(mut self, value: String) -> Self {
        self.codeowner = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn commit_sha(mut self, value: String) -> Self {
        self.commit_sha = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn flag(mut self, value: String) -> Self {
        self.flag = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn pr_number(mut self, value: i64) -> Self {
        self.pr_number = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn repository_id(mut self, value: String) -> Self {
        self.repository_id = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn repository_url(mut self, value: String) -> Self {
        self.repository_url = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn service(mut self, value: String) -> Self {
        self.service = Some(value);
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

impl Default for FilesCoverageRequestAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for FilesCoverageRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FilesCoverageRequestAttributesVisitor;
        impl<'a> Visitor<'a> for FilesCoverageRequestAttributesVisitor {
            type Value = FilesCoverageRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut branch: Option<String> = None;
                let mut changed_only: Option<bool> = None;
                let mut codeowner: Option<String> = None;
                let mut commit_sha: Option<String> = None;
                let mut flag: Option<String> = None;
                let mut pr_number: Option<i64> = None;
                let mut repository_id: Option<String> = None;
                let mut repository_url: Option<String> = None;
                let mut service: Option<String> = None;
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
                        "changed_only" => {
                            if v.is_null() {
                                continue;
                            }
                            changed_only =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "codeowner" => {
                            if v.is_null() {
                                continue;
                            }
                            codeowner = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "commit_sha" => {
                            if v.is_null() {
                                continue;
                            }
                            commit_sha = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "flag" => {
                            if v.is_null() {
                                continue;
                            }
                            flag = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "pr_number" => {
                            if v.is_null() {
                                continue;
                            }
                            pr_number = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "repository_id" => {
                            if v.is_null() {
                                continue;
                            }
                            repository_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "repository_url" => {
                            if v.is_null() {
                                continue;
                            }
                            repository_url =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "service" => {
                            if v.is_null() {
                                continue;
                            }
                            service = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                #[allow(deprecated)]
                let content = FilesCoverageRequestAttributes {
                    branch,
                    changed_only,
                    codeowner,
                    commit_sha,
                    flag,
                    pr_number,
                    repository_id,
                    repository_url,
                    service,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FilesCoverageRequestAttributesVisitor)
    }
}

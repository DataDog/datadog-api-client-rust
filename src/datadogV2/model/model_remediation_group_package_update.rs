// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A package to update in a repository, for a group of library vulnerabilities (`sca.package_update`). The group
/// gathers the open vulnerabilities of direct dependencies that have a recommended fix, and groups them by
/// `@git.repository_id`, `@vulnerability.stack.ecosystem`, and `@remediation.recommended.name`.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RemediationGroupPackageUpdate {
    /// Number of distinct values of `@advisory.id` across the group's findings.
    #[serde(rename = "advisory_count")]
    pub advisory_count: i64,
    /// Package ecosystem, the findings' `@vulnerability.stack.ecosystem`.
    #[serde(rename = "ecosystem")]
    pub ecosystem: String,
    /// Number of distinct values of `@advisory.id` across the group's findings that have
    /// `@risk.has_exploit_available` set to `true`.
    #[serde(rename = "exploitable_cve_count")]
    pub exploitable_cve_count: i64,
    /// Number of distinct values of `@package.declaration.block.filename`, the files that declare the package.
    #[serde(rename = "file_count")]
    pub file_count: i64,
    /// Name of the package to update, the findings' `@remediation.recommended.name`.
    #[serde(rename = "package_name")]
    pub package_name: String,
    /// Repository that depends on the package, the findings' `@git.repository_id`.
    #[serde(rename = "repository_id")]
    pub repository_id: String,
    /// Version to update the package to, the highest `@remediation.recommended.version` across the group's
    /// findings. Omitted when no version is recommended or when the recommended versions can't be ordered.
    #[serde(rename = "target_version")]
    pub target_version: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RemediationGroupPackageUpdate {
    pub fn new(
        advisory_count: i64,
        ecosystem: String,
        exploitable_cve_count: i64,
        file_count: i64,
        package_name: String,
        repository_id: String,
    ) -> RemediationGroupPackageUpdate {
        RemediationGroupPackageUpdate {
            advisory_count,
            ecosystem,
            exploitable_cve_count,
            file_count,
            package_name,
            repository_id,
            target_version: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn target_version(mut self, value: String) -> Self {
        self.target_version = Some(value);
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

impl<'de> Deserialize<'de> for RemediationGroupPackageUpdate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RemediationGroupPackageUpdateVisitor;
        impl<'a> Visitor<'a> for RemediationGroupPackageUpdateVisitor {
            type Value = RemediationGroupPackageUpdate;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut advisory_count: Option<i64> = None;
                let mut ecosystem: Option<String> = None;
                let mut exploitable_cve_count: Option<i64> = None;
                let mut file_count: Option<i64> = None;
                let mut package_name: Option<String> = None;
                let mut repository_id: Option<String> = None;
                let mut target_version: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "advisory_count" => {
                            advisory_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ecosystem" => {
                            ecosystem = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "exploitable_cve_count" => {
                            exploitable_cve_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "file_count" => {
                            file_count = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "package_name" => {
                            package_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "repository_id" => {
                            repository_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "target_version" => {
                            if v.is_null() {
                                continue;
                            }
                            target_version =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let advisory_count =
                    advisory_count.ok_or_else(|| M::Error::missing_field("advisory_count"))?;
                let ecosystem = ecosystem.ok_or_else(|| M::Error::missing_field("ecosystem"))?;
                let exploitable_cve_count = exploitable_cve_count
                    .ok_or_else(|| M::Error::missing_field("exploitable_cve_count"))?;
                let file_count = file_count.ok_or_else(|| M::Error::missing_field("file_count"))?;
                let package_name =
                    package_name.ok_or_else(|| M::Error::missing_field("package_name"))?;
                let repository_id =
                    repository_id.ok_or_else(|| M::Error::missing_field("repository_id"))?;

                let content = RemediationGroupPackageUpdate {
                    advisory_count,
                    ecosystem,
                    exploitable_cve_count,
                    file_count,
                    package_name,
                    repository_id,
                    target_version,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RemediationGroupPackageUpdateVisitor)
    }
}

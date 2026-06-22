// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for requesting code coverage summary for a branch.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct BranchCoverageSummaryRequestAttributes {
    /// The branch name.
    #[serde(rename = "branch")]
    pub branch: String,
    /// Deprecated: use `repository_url` instead. The repository URL.
    #[deprecated]
    #[serde(rename = "repository_id")]
    pub repository_id: Option<String>,
    /// The repository URL. Accepts a full URL with or without a scheme (for example, `<https://github.com/org/repo`> or `github.com/org/repo`).
    #[serde(rename = "repository_url")]
    pub repository_url: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl BranchCoverageSummaryRequestAttributes {
    pub fn new(branch: String) -> BranchCoverageSummaryRequestAttributes {
        #[allow(deprecated)]
        BranchCoverageSummaryRequestAttributes {
            branch,
            repository_id: None,
            repository_url: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
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

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for BranchCoverageSummaryRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BranchCoverageSummaryRequestAttributesVisitor;
        impl<'a> Visitor<'a> for BranchCoverageSummaryRequestAttributesVisitor {
            type Value = BranchCoverageSummaryRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut branch: Option<String> = None;
                let mut repository_id: Option<String> = None;
                let mut repository_url: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "branch" => {
                            branch = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let branch = branch.ok_or_else(|| M::Error::missing_field("branch"))?;

                #[allow(deprecated)]
                let content = BranchCoverageSummaryRequestAttributes {
                    branch,
                    repository_id,
                    repository_url,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(BranchCoverageSummaryRequestAttributesVisitor)
    }
}

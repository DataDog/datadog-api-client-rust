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
    /// The repository identifier.
    #[serde(rename = "repository_id")]
    pub repository_id: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl BranchCoverageSummaryRequestAttributes {
    pub fn new(branch: String, repository_id: String) -> BranchCoverageSummaryRequestAttributes {
        BranchCoverageSummaryRequestAttributes {
            branch,
            repository_id,
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
                            repository_id =
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
                let repository_id =
                    repository_id.ok_or_else(|| M::Error::missing_field("repository_id"))?;

                let content = BranchCoverageSummaryRequestAttributes {
                    branch,
                    repository_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(BranchCoverageSummaryRequestAttributesVisitor)
    }
}

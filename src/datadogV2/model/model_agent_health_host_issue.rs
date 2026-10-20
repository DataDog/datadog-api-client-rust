// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// An active Agent Health issue associated with a resource.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AgentHealthHostIssue {
    /// The category used by the Agent to group and reconcile the issue.
    #[serde(rename = "category")]
    pub category: String,
    /// The unique identifier of the issue.
    #[serde(rename = "issue_id")]
    pub issue_id: String,
    /// The human-readable name of the issue.
    #[serde(rename = "issue_name")]
    pub issue_name: String,
    /// The machine-readable type of the issue.
    #[serde(rename = "issue_type")]
    pub issue_type: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AgentHealthHostIssue {
    pub fn new(
        category: String,
        issue_id: String,
        issue_name: String,
        issue_type: String,
    ) -> AgentHealthHostIssue {
        AgentHealthHostIssue {
            category,
            issue_id,
            issue_name,
            issue_type,
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

impl<'de> Deserialize<'de> for AgentHealthHostIssue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AgentHealthHostIssueVisitor;
        impl<'a> Visitor<'a> for AgentHealthHostIssueVisitor {
            type Value = AgentHealthHostIssue;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut category: Option<String> = None;
                let mut issue_id: Option<String> = None;
                let mut issue_name: Option<String> = None;
                let mut issue_type: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "category" => {
                            category = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "issue_id" => {
                            issue_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "issue_name" => {
                            issue_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "issue_type" => {
                            issue_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let category = category.ok_or_else(|| M::Error::missing_field("category"))?;
                let issue_id = issue_id.ok_or_else(|| M::Error::missing_field("issue_id"))?;
                let issue_name = issue_name.ok_or_else(|| M::Error::missing_field("issue_name"))?;
                let issue_type = issue_type.ok_or_else(|| M::Error::missing_field("issue_type"))?;

                let content = AgentHealthHostIssue {
                    category,
                    issue_id,
                    issue_name,
                    issue_type,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AgentHealthHostIssueVisitor)
    }
}

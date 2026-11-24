// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Pagination details and counts for the matching Agent Health issues.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AgentHealthIssuesResponsePage {
    /// The number of open issues matching the filters in each issue-type group.
    #[serde(rename = "counts_by_issue_type")]
    pub counts_by_issue_type: Option<std::collections::BTreeMap<String, i64>>,
    /// The maximum number of issues returned, echoing the requested `page[limit]`.
    #[serde(rename = "limit")]
    pub limit: i64,
    /// The number of issues skipped, echoing the requested `page[offset]`.
    #[serde(rename = "offset")]
    pub offset: i64,
    /// The total number of open issues matching the filters.
    #[serde(rename = "total_count")]
    pub total_count: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AgentHealthIssuesResponsePage {
    pub fn new(limit: i64, offset: i64, total_count: i64) -> AgentHealthIssuesResponsePage {
        AgentHealthIssuesResponsePage {
            counts_by_issue_type: None,
            limit,
            offset,
            total_count,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn counts_by_issue_type(mut self, value: std::collections::BTreeMap<String, i64>) -> Self {
        self.counts_by_issue_type = Some(value);
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

impl<'de> Deserialize<'de> for AgentHealthIssuesResponsePage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AgentHealthIssuesResponsePageVisitor;
        impl<'a> Visitor<'a> for AgentHealthIssuesResponsePageVisitor {
            type Value = AgentHealthIssuesResponsePage;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut counts_by_issue_type: Option<std::collections::BTreeMap<String, i64>> =
                    None;
                let mut limit: Option<i64> = None;
                let mut offset: Option<i64> = None;
                let mut total_count: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "counts_by_issue_type" => {
                            if v.is_null() {
                                continue;
                            }
                            counts_by_issue_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "limit" => {
                            limit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "offset" => {
                            offset = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_count" => {
                            total_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let limit = limit.ok_or_else(|| M::Error::missing_field("limit"))?;
                let offset = offset.ok_or_else(|| M::Error::missing_field("offset"))?;
                let total_count =
                    total_count.ok_or_else(|| M::Error::missing_field("total_count"))?;

                let content = AgentHealthIssuesResponsePage {
                    counts_by_issue_type,
                    limit,
                    offset,
                    total_count,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AgentHealthIssuesResponsePageVisitor)
    }
}

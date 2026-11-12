// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of an open Agent Health issue, including available remediation.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AgentHealthIssueDetailAttributes {
    /// The category of the issue, such as `configuration`. Omitted when not set.
    #[serde(rename = "category")]
    pub category: Option<String>,
    /// The description of the issue. Omitted when empty.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// The time the issue was first seen, in RFC 3339 format. Omitted when empty.
    #[serde(rename = "first_seen_at")]
    pub first_seen_at: Option<chrono::DateTime<chrono::Utc>>,
    /// The human-readable name of the issue.
    #[serde(rename = "issue_name")]
    pub issue_name: String,
    /// The machine-readable type of the issue.
    #[serde(rename = "issue_type")]
    pub issue_type: String,
    /// Remediation for the issue. Omitted when no remediation is available.
    #[serde(rename = "remediation")]
    pub remediation: Option<crate::datadogV2::model::AgentHealthRemediation>,
    /// The identifier of the affected resource, interpreted according to `resource_kind`: hostname for `host`, DaemonSet UID for `deployment` (node Agent), cluster UUID for `cluster`, hostname of the host running the Cluster Agent for `cluster_agent`, or OpenTelemetry collector ID for `otel`.
    #[serde(rename = "resource_id")]
    pub resource_id: String,
    /// The kind of affected resource. Known values are `host`, `deployment`, `cluster`, `cluster_agent`, and `otel`.
    #[serde(rename = "resource_kind")]
    pub resource_kind: String,
    /// The type of the affected resource. Omitted when empty.
    #[serde(rename = "resource_type")]
    pub resource_type: Option<String>,
    /// The severity of the issue. Known values are `low`, `medium`, `high`, and `unknown`.
    #[serde(rename = "severity")]
    pub severity: String,
    /// The state of the issue. Only issues with state `open` are returned.
    #[serde(rename = "state")]
    pub state: String,
    /// Tags associated with the issue. Omitted when empty.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// The title of the issue. Omitted when empty.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// The time the issue was last updated, in RFC 3339 format. Omitted when empty.
    #[serde(rename = "updated_at")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AgentHealthIssueDetailAttributes {
    pub fn new(
        issue_name: String,
        issue_type: String,
        resource_id: String,
        resource_kind: String,
        severity: String,
        state: String,
    ) -> AgentHealthIssueDetailAttributes {
        AgentHealthIssueDetailAttributes {
            category: None,
            description: None,
            first_seen_at: None,
            issue_name,
            issue_type,
            remediation: None,
            resource_id,
            resource_kind,
            resource_type: None,
            severity,
            state,
            tags: None,
            title: None,
            updated_at: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn category(mut self, value: String) -> Self {
        self.category = Some(value);
        self
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn first_seen_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.first_seen_at = Some(value);
        self
    }

    pub fn remediation(mut self, value: crate::datadogV2::model::AgentHealthRemediation) -> Self {
        self.remediation = Some(value);
        self
    }

    pub fn resource_type(mut self, value: String) -> Self {
        self.resource_type = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn title(mut self, value: String) -> Self {
        self.title = Some(value);
        self
    }

    pub fn updated_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.updated_at = Some(value);
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

impl<'de> Deserialize<'de> for AgentHealthIssueDetailAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AgentHealthIssueDetailAttributesVisitor;
        impl<'a> Visitor<'a> for AgentHealthIssueDetailAttributesVisitor {
            type Value = AgentHealthIssueDetailAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut category: Option<String> = None;
                let mut description: Option<String> = None;
                let mut first_seen_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut issue_name: Option<String> = None;
                let mut issue_type: Option<String> = None;
                let mut remediation: Option<crate::datadogV2::model::AgentHealthRemediation> = None;
                let mut resource_id: Option<String> = None;
                let mut resource_kind: Option<String> = None;
                let mut resource_type: Option<String> = None;
                let mut severity: Option<String> = None;
                let mut state: Option<String> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut title: Option<String> = None;
                let mut updated_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "category" => {
                            if v.is_null() {
                                continue;
                            }
                            category = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "first_seen_at" => {
                            if v.is_null() {
                                continue;
                            }
                            first_seen_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "issue_name" => {
                            issue_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "issue_type" => {
                            issue_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "remediation" => {
                            if v.is_null() {
                                continue;
                            }
                            remediation =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resource_id" => {
                            resource_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resource_kind" => {
                            resource_kind =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resource_type" => {
                            if v.is_null() {
                                continue;
                            }
                            resource_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "severity" => {
                            severity = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "state" => {
                            state = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "title" => {
                            if v.is_null() {
                                continue;
                            }
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_at" => {
                            if v.is_null() {
                                continue;
                            }
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let issue_name = issue_name.ok_or_else(|| M::Error::missing_field("issue_name"))?;
                let issue_type = issue_type.ok_or_else(|| M::Error::missing_field("issue_type"))?;
                let resource_id =
                    resource_id.ok_or_else(|| M::Error::missing_field("resource_id"))?;
                let resource_kind =
                    resource_kind.ok_or_else(|| M::Error::missing_field("resource_kind"))?;
                let severity = severity.ok_or_else(|| M::Error::missing_field("severity"))?;
                let state = state.ok_or_else(|| M::Error::missing_field("state"))?;

                let content = AgentHealthIssueDetailAttributes {
                    category,
                    description,
                    first_seen_at,
                    issue_name,
                    issue_type,
                    remediation,
                    resource_id,
                    resource_kind,
                    resource_type,
                    severity,
                    state,
                    tags,
                    title,
                    updated_at,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AgentHealthIssueDetailAttributesVisitor)
    }
}

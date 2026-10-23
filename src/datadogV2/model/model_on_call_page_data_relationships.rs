// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The relationships of an On-Call Page.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OnCallPageDataRelationships {
    /// Reference to the entity that performed an action on the page.
    #[serde(rename = "author")]
    pub author: Option<crate::datadogV2::model::OnCallPageAuthorRelationship>,
    /// Reference to the entity that performed an action on the page.
    #[serde(rename = "closed_by")]
    pub closed_by: Option<crate::datadogV2::model::OnCallPageAuthorRelationship>,
    /// The Jira issue linked to the page.
    #[serde(rename = "jira_issue")]
    pub jira_issue: Option<crate::datadogV2::model::OnCallPageJiraIssueRelationship>,
    /// Links between the page and other Datadog resources, such as incidents. Always empty when listing pages.
    #[serde(rename = "links")]
    pub links: Option<crate::datadogV2::model::OnCallPageLinksRelationship>,
    /// The users responding to the page.
    #[serde(rename = "responders")]
    pub responders: Option<crate::datadogV2::model::OnCallPageRespondersRelationship>,
    /// The next action scheduled on the page, such as an escalation.
    #[serde(rename = "scheduled_action")]
    pub scheduled_action: Option<crate::datadogV2::model::OnCallPageScheduledActionRelationship>,
    /// The ServiceNow incident linked to the page.
    #[serde(rename = "service_now_incident")]
    pub service_now_incident:
        Option<crate::datadogV2::model::OnCallPageServiceNowIncidentRelationship>,
    /// The team, user, or cross-organization handle that was paged.
    #[serde(rename = "target")]
    pub target: Option<crate::datadogV2::model::OnCallPageTargetRelationship>,
    /// The team routing rule that matched the page.
    #[serde(rename = "team_routing_rule")]
    pub team_routing_rule: Option<crate::datadogV2::model::OnCallPageTeamRoutingRuleRelationship>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OnCallPageDataRelationships {
    pub fn new() -> OnCallPageDataRelationships {
        OnCallPageDataRelationships {
            author: None,
            closed_by: None,
            jira_issue: None,
            links: None,
            responders: None,
            scheduled_action: None,
            service_now_incident: None,
            target: None,
            team_routing_rule: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn author(mut self, value: crate::datadogV2::model::OnCallPageAuthorRelationship) -> Self {
        self.author = Some(value);
        self
    }

    pub fn closed_by(
        mut self,
        value: crate::datadogV2::model::OnCallPageAuthorRelationship,
    ) -> Self {
        self.closed_by = Some(value);
        self
    }

    pub fn jira_issue(
        mut self,
        value: crate::datadogV2::model::OnCallPageJiraIssueRelationship,
    ) -> Self {
        self.jira_issue = Some(value);
        self
    }

    pub fn links(mut self, value: crate::datadogV2::model::OnCallPageLinksRelationship) -> Self {
        self.links = Some(value);
        self
    }

    pub fn responders(
        mut self,
        value: crate::datadogV2::model::OnCallPageRespondersRelationship,
    ) -> Self {
        self.responders = Some(value);
        self
    }

    pub fn scheduled_action(
        mut self,
        value: crate::datadogV2::model::OnCallPageScheduledActionRelationship,
    ) -> Self {
        self.scheduled_action = Some(value);
        self
    }

    pub fn service_now_incident(
        mut self,
        value: crate::datadogV2::model::OnCallPageServiceNowIncidentRelationship,
    ) -> Self {
        self.service_now_incident = Some(value);
        self
    }

    pub fn target(mut self, value: crate::datadogV2::model::OnCallPageTargetRelationship) -> Self {
        self.target = Some(value);
        self
    }

    pub fn team_routing_rule(
        mut self,
        value: crate::datadogV2::model::OnCallPageTeamRoutingRuleRelationship,
    ) -> Self {
        self.team_routing_rule = Some(value);
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

impl Default for OnCallPageDataRelationships {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for OnCallPageDataRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OnCallPageDataRelationshipsVisitor;
        impl<'a> Visitor<'a> for OnCallPageDataRelationshipsVisitor {
            type Value = OnCallPageDataRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut author: Option<crate::datadogV2::model::OnCallPageAuthorRelationship> =
                    None;
                let mut closed_by: Option<crate::datadogV2::model::OnCallPageAuthorRelationship> =
                    None;
                let mut jira_issue: Option<
                    crate::datadogV2::model::OnCallPageJiraIssueRelationship,
                > = None;
                let mut links: Option<crate::datadogV2::model::OnCallPageLinksRelationship> = None;
                let mut responders: Option<
                    crate::datadogV2::model::OnCallPageRespondersRelationship,
                > = None;
                let mut scheduled_action: Option<
                    crate::datadogV2::model::OnCallPageScheduledActionRelationship,
                > = None;
                let mut service_now_incident: Option<
                    crate::datadogV2::model::OnCallPageServiceNowIncidentRelationship,
                > = None;
                let mut target: Option<crate::datadogV2::model::OnCallPageTargetRelationship> =
                    None;
                let mut team_routing_rule: Option<
                    crate::datadogV2::model::OnCallPageTeamRoutingRuleRelationship,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "author" => {
                            if v.is_null() {
                                continue;
                            }
                            author = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "closed_by" => {
                            if v.is_null() {
                                continue;
                            }
                            closed_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "jira_issue" => {
                            if v.is_null() {
                                continue;
                            }
                            jira_issue = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "links" => {
                            if v.is_null() {
                                continue;
                            }
                            links = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "responders" => {
                            if v.is_null() {
                                continue;
                            }
                            responders = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "scheduled_action" => {
                            if v.is_null() {
                                continue;
                            }
                            scheduled_action =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "service_now_incident" => {
                            if v.is_null() {
                                continue;
                            }
                            service_now_incident =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "target" => {
                            if v.is_null() {
                                continue;
                            }
                            target = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "team_routing_rule" => {
                            if v.is_null() {
                                continue;
                            }
                            team_routing_rule =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = OnCallPageDataRelationships {
                    author,
                    closed_by,
                    jira_issue,
                    links,
                    responders,
                    scheduled_action,
                    service_now_incident,
                    target,
                    team_routing_rule,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OnCallPageDataRelationshipsVisitor)
    }
}

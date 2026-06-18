// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The action to take when the ticket creation rule matches a finding.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TicketCreationRuleAction {
    /// The UUID of the default assignee for created tickets.
    #[serde(rename = "assignee_id")]
    pub assignee_id: Option<uuid::Uuid>,
    /// Custom fields of the Jira issue to create. For the list of available fields, see [Jira documentation](<https://developer.atlassian.com/cloud/jira/platform/rest/v2/api-group-issues/#api-rest-api-2-issue-createmeta-projectidorkey-issuetypes-issuetypeid-get>).
    #[serde(rename = "fields")]
    pub fields: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// The maximum number of tickets the rule may create per day. If exceeded, one final ticket will be created, explaining the limit was hit and link back to the responsible rule.
    #[serde(rename = "max_tickets_per_day")]
    pub max_tickets_per_day: i64,
    /// The UUID of the case management project.
    #[serde(rename = "project_id")]
    pub project_id: uuid::Uuid,
    /// The ticketing system to create tickets in.
    #[serde(rename = "target")]
    pub target: crate::datadogV2::model::TicketCreationTarget,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TicketCreationRuleAction {
    pub fn new(
        max_tickets_per_day: i64,
        project_id: uuid::Uuid,
        target: crate::datadogV2::model::TicketCreationTarget,
    ) -> TicketCreationRuleAction {
        TicketCreationRuleAction {
            assignee_id: None,
            fields: None,
            max_tickets_per_day,
            project_id,
            target,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn assignee_id(mut self, value: uuid::Uuid) -> Self {
        self.assignee_id = Some(value);
        self
    }

    pub fn fields(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.fields = Some(value);
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

impl<'de> Deserialize<'de> for TicketCreationRuleAction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TicketCreationRuleActionVisitor;
        impl<'a> Visitor<'a> for TicketCreationRuleActionVisitor {
            type Value = TicketCreationRuleAction;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut assignee_id: Option<uuid::Uuid> = None;
                let mut fields: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut max_tickets_per_day: Option<i64> = None;
                let mut project_id: Option<uuid::Uuid> = None;
                let mut target: Option<crate::datadogV2::model::TicketCreationTarget> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "assignee_id" => {
                            if v.is_null() {
                                continue;
                            }
                            assignee_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "fields" => {
                            if v.is_null() {
                                continue;
                            }
                            fields = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "max_tickets_per_day" => {
                            max_tickets_per_day =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "project_id" => {
                            project_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "target" => {
                            target = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _target) = target {
                                match _target {
                                    crate::datadogV2::model::TicketCreationTarget::UnparsedObject(_target) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let max_tickets_per_day = max_tickets_per_day
                    .ok_or_else(|| M::Error::missing_field("max_tickets_per_day"))?;
                let project_id = project_id.ok_or_else(|| M::Error::missing_field("project_id"))?;
                let target = target.ok_or_else(|| M::Error::missing_field("target"))?;

                let content = TicketCreationRuleAction {
                    assignee_id,
                    fields,
                    max_tickets_per_day,
                    project_id,
                    target,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TicketCreationRuleActionVisitor)
    }
}

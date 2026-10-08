// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Reroutes the page to another team, which then evaluates it against its own routing rules. Each routing rule can include this action only once. It can be combined only with `send_slack_message` and `send_teams_message` actions. It can't be used with `escalation_policy` or `workflow` actions, or when the routing rule sets `policy_id`.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RoutingRuleRerouteToTeamAction {
    /// The ID of the team to reroute the page to.
    #[serde(rename = "destination_team_id")]
    pub destination_team_id: uuid::Uuid,
    /// Indicates that the action reroutes the page to another team's routing rules.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::RoutingRuleRerouteToTeamActionType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RoutingRuleRerouteToTeamAction {
    pub fn new(
        destination_team_id: uuid::Uuid,
        type_: crate::datadogV2::model::RoutingRuleRerouteToTeamActionType,
    ) -> RoutingRuleRerouteToTeamAction {
        RoutingRuleRerouteToTeamAction {
            destination_team_id,
            type_,
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

impl<'de> Deserialize<'de> for RoutingRuleRerouteToTeamAction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RoutingRuleRerouteToTeamActionVisitor;
        impl<'a> Visitor<'a> for RoutingRuleRerouteToTeamActionVisitor {
            type Value = RoutingRuleRerouteToTeamAction;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut destination_team_id: Option<uuid::Uuid> = None;
                let mut type_: Option<crate::datadogV2::model::RoutingRuleRerouteToTeamActionType> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "destination_team_id" => {
                            destination_team_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::RoutingRuleRerouteToTeamActionType::UnparsedObject(_type_) => {
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
                let destination_team_id = destination_team_id
                    .ok_or_else(|| M::Error::missing_field("destination_team_id"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = RoutingRuleRerouteToTeamAction {
                    destination_team_id,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RoutingRuleRerouteToTeamActionVisitor)
    }
}

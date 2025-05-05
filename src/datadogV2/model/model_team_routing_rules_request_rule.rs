// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Defines an individual routing rule item that contains the rule data for the request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TeamRoutingRulesRequestRule {
    /// Specifies the list of actions to perform when the routing rule is matched.
    #[serde(rename = "actions")]
    pub actions: Option<Vec<crate::datadogV2::model::RoutingRuleAction>>,
    /// Identifies the policy to be applied when this routing rule matches.
    #[serde(rename = "policy_id")]
    pub policy_id: Option<String>,
    /// Defines the query or condition that triggers this routing rule.
    #[serde(rename = "query")]
    pub query: Option<String>,
    /// Holds time zone information and a list of time restrictions for a routing rule.
    #[serde(rename = "time_restriction")]
    pub time_restriction: Option<crate::datadogV2::model::TimeRestrictions>,
    /// Specifies the level of urgency for a routing rule (low, high, or dynamic).
    #[serde(rename = "urgency")]
    pub urgency: Option<crate::datadogV2::model::Urgency>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TeamRoutingRulesRequestRule {
    pub fn new() -> TeamRoutingRulesRequestRule {
        TeamRoutingRulesRequestRule {
            actions: None,
            policy_id: None,
            query: None,
            time_restriction: None,
            urgency: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn actions(mut self, value: Vec<crate::datadogV2::model::RoutingRuleAction>) -> Self {
        self.actions = Some(value);
        self
    }

    pub fn policy_id(mut self, value: String) -> Self {
        self.policy_id = Some(value);
        self
    }

    pub fn query(mut self, value: String) -> Self {
        self.query = Some(value);
        self
    }

    pub fn time_restriction(mut self, value: crate::datadogV2::model::TimeRestrictions) -> Self {
        self.time_restriction = Some(value);
        self
    }

    pub fn urgency(mut self, value: crate::datadogV2::model::Urgency) -> Self {
        self.urgency = Some(value);
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

impl Default for TeamRoutingRulesRequestRule {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TeamRoutingRulesRequestRule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TeamRoutingRulesRequestRuleVisitor;
        impl<'a> Visitor<'a> for TeamRoutingRulesRequestRuleVisitor {
            type Value = TeamRoutingRulesRequestRule;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut actions: Option<Vec<crate::datadogV2::model::RoutingRuleAction>> = None;
                let mut policy_id: Option<String> = None;
                let mut query: Option<String> = None;
                let mut time_restriction: Option<crate::datadogV2::model::TimeRestrictions> = None;
                let mut urgency: Option<crate::datadogV2::model::Urgency> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "actions" => {
                            if v.is_null() {
                                continue;
                            }
                            actions = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "policy_id" => {
                            if v.is_null() {
                                continue;
                            }
                            policy_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query" => {
                            if v.is_null() {
                                continue;
                            }
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "time_restriction" => {
                            if v.is_null() {
                                continue;
                            }
                            time_restriction =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "urgency" => {
                            if v.is_null() {
                                continue;
                            }
                            urgency = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _urgency) = urgency {
                                match _urgency {
                                    crate::datadogV2::model::Urgency::UnparsedObject(_urgency) => {
                                        _unparsed = true;
                                    }
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

                let content = TeamRoutingRulesRequestRule {
                    actions,
                    policy_id,
                    query,
                    time_restriction,
                    urgency,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TeamRoutingRulesRequestRuleVisitor)
    }
}

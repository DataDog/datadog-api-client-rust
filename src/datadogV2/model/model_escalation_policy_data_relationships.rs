// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Represents the relationships for an escalation policy, including references to steps and teams.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct EscalationPolicyDataRelationships {
    /// Defines the relationship to a collection of steps within an escalation policy. Contains an array of step data references.
    #[serde(rename = "steps")]
    pub steps: crate::datadogV2::model::EscalationPolicyDataRelationshipsSteps,
    /// Defines the relationship to a collection of teams within an escalation policy. Contains an array of team data references.
    #[serde(rename = "teams")]
    pub teams: Option<crate::datadogV2::model::EscalationPolicyDataRelationshipsTeams>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl EscalationPolicyDataRelationships {
    pub fn new(
        steps: crate::datadogV2::model::EscalationPolicyDataRelationshipsSteps,
    ) -> EscalationPolicyDataRelationships {
        EscalationPolicyDataRelationships {
            steps,
            teams: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn teams(
        mut self,
        value: crate::datadogV2::model::EscalationPolicyDataRelationshipsTeams,
    ) -> Self {
        self.teams = Some(value);
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

impl<'de> Deserialize<'de> for EscalationPolicyDataRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EscalationPolicyDataRelationshipsVisitor;
        impl<'a> Visitor<'a> for EscalationPolicyDataRelationshipsVisitor {
            type Value = EscalationPolicyDataRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut steps: Option<
                    crate::datadogV2::model::EscalationPolicyDataRelationshipsSteps,
                > = None;
                let mut teams: Option<
                    crate::datadogV2::model::EscalationPolicyDataRelationshipsTeams,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "steps" => {
                            steps = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "teams" => {
                            if v.is_null() {
                                continue;
                            }
                            teams = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let steps = steps.ok_or_else(|| M::Error::missing_field("steps"))?;

                let content = EscalationPolicyDataRelationships {
                    steps,
                    teams,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(EscalationPolicyDataRelationshipsVisitor)
    }
}

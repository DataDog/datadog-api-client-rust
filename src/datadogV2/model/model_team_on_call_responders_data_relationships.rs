// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Relationship objects linked to a team's on-call responder configuration, including escalations and responders.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TeamOnCallRespondersDataRelationships {
    /// Defines the escalation policy steps linked to the team's on-call configuration.
    #[serde(rename = "escalations")]
    pub escalations:
        Option<crate::datadogV2::model::TeamOnCallRespondersDataRelationshipsEscalations>,
    /// Defines the list of users assigned as on-call responders for the team.
    #[serde(rename = "responders")]
    pub responders:
        Option<crate::datadogV2::model::TeamOnCallRespondersDataRelationshipsResponders>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TeamOnCallRespondersDataRelationships {
    pub fn new() -> TeamOnCallRespondersDataRelationships {
        TeamOnCallRespondersDataRelationships {
            escalations: None,
            responders: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn escalations(
        mut self,
        value: crate::datadogV2::model::TeamOnCallRespondersDataRelationshipsEscalations,
    ) -> Self {
        self.escalations = Some(value);
        self
    }

    pub fn responders(
        mut self,
        value: crate::datadogV2::model::TeamOnCallRespondersDataRelationshipsResponders,
    ) -> Self {
        self.responders = Some(value);
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

impl Default for TeamOnCallRespondersDataRelationships {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TeamOnCallRespondersDataRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TeamOnCallRespondersDataRelationshipsVisitor;
        impl<'a> Visitor<'a> for TeamOnCallRespondersDataRelationshipsVisitor {
            type Value = TeamOnCallRespondersDataRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut escalations: Option<
                    crate::datadogV2::model::TeamOnCallRespondersDataRelationshipsEscalations,
                > = None;
                let mut responders: Option<
                    crate::datadogV2::model::TeamOnCallRespondersDataRelationshipsResponders,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "escalations" => {
                            if v.is_null() {
                                continue;
                            }
                            escalations =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "responders" => {
                            if v.is_null() {
                                continue;
                            }
                            responders = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = TeamOnCallRespondersDataRelationships {
                    escalations,
                    responders,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TeamOnCallRespondersDataRelationshipsVisitor)
    }
}

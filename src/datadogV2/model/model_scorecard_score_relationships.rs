// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Relationships for a scorecard score, depending on the aggregation type.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ScorecardScoreRelationships {
    /// A relationship item for a score.
    #[serde(rename = "entity")]
    pub entity: Option<crate::datadogV2::model::ScorecardScoreRelationshipItem>,
    /// A relationship item for a score.
    #[serde(rename = "rule")]
    pub rule: Option<crate::datadogV2::model::ScorecardScoreRelationshipItem>,
    /// A relationship item for a score.
    #[serde(rename = "scorecard")]
    pub scorecard: Option<crate::datadogV2::model::ScorecardScoreRelationshipItem>,
    /// A relationship item for a score.
    #[serde(rename = "service")]
    pub service: Option<crate::datadogV2::model::ScorecardScoreRelationshipItem>,
    /// A relationship item for a score.
    #[serde(rename = "team")]
    pub team: Option<crate::datadogV2::model::ScorecardScoreRelationshipItem>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ScorecardScoreRelationships {
    pub fn new() -> ScorecardScoreRelationships {
        ScorecardScoreRelationships {
            entity: None,
            rule: None,
            scorecard: None,
            service: None,
            team: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn entity(
        mut self,
        value: crate::datadogV2::model::ScorecardScoreRelationshipItem,
    ) -> Self {
        self.entity = Some(value);
        self
    }

    pub fn rule(mut self, value: crate::datadogV2::model::ScorecardScoreRelationshipItem) -> Self {
        self.rule = Some(value);
        self
    }

    pub fn scorecard(
        mut self,
        value: crate::datadogV2::model::ScorecardScoreRelationshipItem,
    ) -> Self {
        self.scorecard = Some(value);
        self
    }

    pub fn service(
        mut self,
        value: crate::datadogV2::model::ScorecardScoreRelationshipItem,
    ) -> Self {
        self.service = Some(value);
        self
    }

    pub fn team(mut self, value: crate::datadogV2::model::ScorecardScoreRelationshipItem) -> Self {
        self.team = Some(value);
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

impl Default for ScorecardScoreRelationships {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ScorecardScoreRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ScorecardScoreRelationshipsVisitor;
        impl<'a> Visitor<'a> for ScorecardScoreRelationshipsVisitor {
            type Value = ScorecardScoreRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut entity: Option<crate::datadogV2::model::ScorecardScoreRelationshipItem> =
                    None;
                let mut rule: Option<crate::datadogV2::model::ScorecardScoreRelationshipItem> =
                    None;
                let mut scorecard: Option<crate::datadogV2::model::ScorecardScoreRelationshipItem> =
                    None;
                let mut service: Option<crate::datadogV2::model::ScorecardScoreRelationshipItem> =
                    None;
                let mut team: Option<crate::datadogV2::model::ScorecardScoreRelationshipItem> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "entity" => {
                            if v.is_null() {
                                continue;
                            }
                            entity = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rule" => {
                            if v.is_null() {
                                continue;
                            }
                            rule = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "scorecard" => {
                            if v.is_null() {
                                continue;
                            }
                            scorecard = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "service" => {
                            if v.is_null() {
                                continue;
                            }
                            service = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "team" => {
                            if v.is_null() {
                                continue;
                            }
                            team = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ScorecardScoreRelationships {
                    entity,
                    rule,
                    scorecard,
                    service,
                    team,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ScorecardScoreRelationshipsVisitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The entities the investigation found, by role.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityAgentInvestigationKeyEntities {
    /// Entities that performed the activity.
    #[serde(rename = "actors")]
    pub actors: Vec<crate::datadogV2::model::SecurityAgentInvestigationEntity>,
    /// Entities the activity was aimed at.
    #[serde(rename = "targets")]
    pub targets: Vec<crate::datadogV2::model::SecurityAgentInvestigationEntity>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityAgentInvestigationKeyEntities {
    pub fn new(
        actors: Vec<crate::datadogV2::model::SecurityAgentInvestigationEntity>,
        targets: Vec<crate::datadogV2::model::SecurityAgentInvestigationEntity>,
    ) -> SecurityAgentInvestigationKeyEntities {
        SecurityAgentInvestigationKeyEntities {
            actors,
            targets,
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

impl<'de> Deserialize<'de> for SecurityAgentInvestigationKeyEntities {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityAgentInvestigationKeyEntitiesVisitor;
        impl<'a> Visitor<'a> for SecurityAgentInvestigationKeyEntitiesVisitor {
            type Value = SecurityAgentInvestigationKeyEntities;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut actors: Option<
                    Vec<crate::datadogV2::model::SecurityAgentInvestigationEntity>,
                > = None;
                let mut targets: Option<
                    Vec<crate::datadogV2::model::SecurityAgentInvestigationEntity>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "actors" => {
                            actors = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "targets" => {
                            targets = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let actors = actors.ok_or_else(|| M::Error::missing_field("actors"))?;
                let targets = targets.ok_or_else(|| M::Error::missing_field("targets"))?;

                let content = SecurityAgentInvestigationKeyEntities {
                    actors,
                    targets,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityAgentInvestigationKeyEntitiesVisitor)
    }
}

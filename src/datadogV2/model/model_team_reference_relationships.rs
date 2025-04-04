// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Collects the key relationship fields for a team reference, specifically on-call users.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TeamReferenceRelationships {
    /// Defines which users are on-call within a team, stored as an array of references.
    #[serde(rename = "oncall_users")]
    pub oncall_users: Option<crate::datadogV2::model::TeamReferenceRelationshipsOncallUsers>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TeamReferenceRelationships {
    pub fn new() -> TeamReferenceRelationships {
        TeamReferenceRelationships {
            oncall_users: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn oncall_users(
        mut self,
        value: crate::datadogV2::model::TeamReferenceRelationshipsOncallUsers,
    ) -> Self {
        self.oncall_users = Some(value);
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

impl Default for TeamReferenceRelationships {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TeamReferenceRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TeamReferenceRelationshipsVisitor;
        impl<'a> Visitor<'a> for TeamReferenceRelationshipsVisitor {
            type Value = TeamReferenceRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut oncall_users: Option<
                    crate::datadogV2::model::TeamReferenceRelationshipsOncallUsers,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "oncall_users" => {
                            if v.is_null() {
                                continue;
                            }
                            oncall_users =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = TeamReferenceRelationships {
                    oncall_users,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TeamReferenceRelationshipsVisitor)
    }
}

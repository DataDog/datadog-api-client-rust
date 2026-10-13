// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Relationships for an on-call schedule override.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OverrideRelationships {
    /// Defines the relationship between an override and one of its associated users.
    #[serde(rename = "overridden_user")]
    pub overridden_user: Option<crate::datadogV2::model::OverrideRelationshipsUser>,
    /// Defines the relationship between an override and the schedule it belongs to.
    #[serde(rename = "schedule")]
    pub schedule: Option<crate::datadogV2::model::OverrideRelationshipsSchedule>,
    /// Defines the relationship between an override and one of its associated users.
    #[serde(rename = "user")]
    pub user: Option<crate::datadogV2::model::OverrideRelationshipsUser>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OverrideRelationships {
    pub fn new() -> OverrideRelationships {
        OverrideRelationships {
            overridden_user: None,
            schedule: None,
            user: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn overridden_user(
        mut self,
        value: crate::datadogV2::model::OverrideRelationshipsUser,
    ) -> Self {
        self.overridden_user = Some(value);
        self
    }

    pub fn schedule(
        mut self,
        value: crate::datadogV2::model::OverrideRelationshipsSchedule,
    ) -> Self {
        self.schedule = Some(value);
        self
    }

    pub fn user(mut self, value: crate::datadogV2::model::OverrideRelationshipsUser) -> Self {
        self.user = Some(value);
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

impl Default for OverrideRelationships {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for OverrideRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OverrideRelationshipsVisitor;
        impl<'a> Visitor<'a> for OverrideRelationshipsVisitor {
            type Value = OverrideRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut overridden_user: Option<
                    crate::datadogV2::model::OverrideRelationshipsUser,
                > = None;
                let mut schedule: Option<crate::datadogV2::model::OverrideRelationshipsSchedule> =
                    None;
                let mut user: Option<crate::datadogV2::model::OverrideRelationshipsUser> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "overridden_user" => {
                            if v.is_null() {
                                continue;
                            }
                            overridden_user =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "schedule" => {
                            if v.is_null() {
                                continue;
                            }
                            schedule = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "user" => {
                            if v.is_null() {
                                continue;
                            }
                            user = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = OverrideRelationships {
                    overridden_user,
                    schedule,
                    user,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OverrideRelationshipsVisitor)
    }
}

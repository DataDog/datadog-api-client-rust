// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The relationships the incident will have with other resources once created.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentCreateRelationships {
    /// Relationship to user.
    #[serialize_always]
    #[serde(rename = "commander_user")]
    pub commander_user: Option<crate::datadogV2::model::NullableRelationshipToUser>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentCreateRelationships {
    pub fn new(
        commander_user: Option<crate::datadogV2::model::NullableRelationshipToUser>,
    ) -> IncidentCreateRelationships {
        IncidentCreateRelationships {
            commander_user,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for IncidentCreateRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentCreateRelationshipsVisitor;
        impl<'a> Visitor<'a> for IncidentCreateRelationshipsVisitor {
            type Value = IncidentCreateRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut commander_user: Option<
                    Option<crate::datadogV2::model::NullableRelationshipToUser>,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "commander_user" => {
                            commander_user =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let commander_user =
                    commander_user.ok_or_else(|| M::Error::missing_field("commander_user"))?;

                let content = IncidentCreateRelationships {
                    commander_user,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentCreateRelationshipsVisitor)
    }
}

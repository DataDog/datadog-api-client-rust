// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The team associated with the membership
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RelationshipToUserTeamTeamData {
    /// The ID of the team associated with the membership
    #[serde(rename = "id")]
    pub id: String,
    /// User team team type
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::UserTeamTeamType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RelationshipToUserTeamTeamData {
    pub fn new(
        id: String,
        type_: crate::datadogV2::model::UserTeamTeamType,
    ) -> RelationshipToUserTeamTeamData {
        RelationshipToUserTeamTeamData {
            id,
            type_,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for RelationshipToUserTeamTeamData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RelationshipToUserTeamTeamDataVisitor;
        impl<'a> Visitor<'a> for RelationshipToUserTeamTeamDataVisitor {
            type Value = RelationshipToUserTeamTeamData;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut id: Option<String> = None;
                let mut type_: Option<crate::datadogV2::model::UserTeamTeamType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::UserTeamTeamType::UnparsedObject(
                                        _type_,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = RelationshipToUserTeamTeamData {
                    id,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RelationshipToUserTeamTeamDataVisitor)
    }
}

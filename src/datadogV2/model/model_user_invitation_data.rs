// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object to create a user invitation.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UserInvitationData {
    /// Relationships data for user invitation.
    #[serde(rename = "relationships")]
    pub relationships: crate::datadogV2::model::UserInvitationRelationships,
    /// User invitations type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::UserInvitationsType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UserInvitationData {
    pub fn new(
        relationships: crate::datadogV2::model::UserInvitationRelationships,
        type_: crate::datadogV2::model::UserInvitationsType,
    ) -> UserInvitationData {
        UserInvitationData {
            relationships,
            type_,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for UserInvitationData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UserInvitationDataVisitor;
        impl<'a> Visitor<'a> for UserInvitationDataVisitor {
            type Value = UserInvitationData;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut relationships: Option<
                    crate::datadogV2::model::UserInvitationRelationships,
                > = None;
                let mut type_: Option<crate::datadogV2::model::UserInvitationsType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "relationships" => {
                            relationships =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::UserInvitationsType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }
                let relationships =
                    relationships.ok_or_else(|| M::Error::missing_field("relationships"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = UserInvitationData {
                    relationships,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UserInvitationDataVisitor)
    }
}

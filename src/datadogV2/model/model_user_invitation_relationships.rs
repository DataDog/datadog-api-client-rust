// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Relationships data for user invitation.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UserInvitationRelationships {
    /// Relationship to user.
    #[serde(rename = "user")]
    pub user: crate::datadogV2::model::RelationshipToUser,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UserInvitationRelationships {
    pub fn new(user: crate::datadogV2::model::RelationshipToUser) -> UserInvitationRelationships {
        UserInvitationRelationships {
            user,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for UserInvitationRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UserInvitationRelationshipsVisitor;
        impl<'a> Visitor<'a> for UserInvitationRelationshipsVisitor {
            type Value = UserInvitationRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut user: Option<crate::datadogV2::model::RelationshipToUser> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "user" => {
                            user = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let user = user.ok_or_else(|| M::Error::missing_field("user"))?;

                let content = UserInvitationRelationships { user, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UserInvitationRelationshipsVisitor)
    }
}

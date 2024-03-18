// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object to invite users to join the organization.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UserInvitationsRequest {
    /// List of user invitations.
    #[serde(rename = "data")]
    pub data: Vec<crate::datadogV2::model::UserInvitationData>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UserInvitationsRequest {
    pub fn new(data: Vec<crate::datadogV2::model::UserInvitationData>) -> UserInvitationsRequest {
        UserInvitationsRequest {
            data,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for UserInvitationsRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UserInvitationsRequestVisitor;
        impl<'a> Visitor<'a> for UserInvitationsRequestVisitor {
            type Value = UserInvitationsRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data: Option<Vec<crate::datadogV2::model::UserInvitationData>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data" => {
                            data = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let data = data.ok_or_else(|| M::Error::missing_field("data"))?;

                let content = UserInvitationsRequest { data, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UserInvitationsRequestVisitor)
    }
}

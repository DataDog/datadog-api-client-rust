// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Relationships of the user object returned by the API.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UserResponseRelationships {
    /// Relationship to an organization.
    #[serde(rename = "org")]
    pub org: Option<crate::datadogV2::model::RelationshipToOrganization>,
    /// Relationship to organizations.
    #[serde(rename = "other_orgs")]
    pub other_orgs: Option<crate::datadogV2::model::RelationshipToOrganizations>,
    /// Relationship to users.
    #[serde(rename = "other_users")]
    pub other_users: Option<crate::datadogV2::model::RelationshipToUsers>,
    /// Relationship to roles.
    #[serde(rename = "roles")]
    pub roles: Option<crate::datadogV2::model::RelationshipToRoles>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UserResponseRelationships {
    pub fn new() -> UserResponseRelationships {
        UserResponseRelationships {
            org: None,
            other_orgs: None,
            other_users: None,
            roles: None,
            _unparsed: false,
        }
    }

    pub fn org(mut self, value: crate::datadogV2::model::RelationshipToOrganization) -> Self {
        self.org = Some(value);
        self
    }

    pub fn other_orgs(
        mut self,
        value: crate::datadogV2::model::RelationshipToOrganizations,
    ) -> Self {
        self.other_orgs = Some(value);
        self
    }

    pub fn other_users(mut self, value: crate::datadogV2::model::RelationshipToUsers) -> Self {
        self.other_users = Some(value);
        self
    }

    pub fn roles(mut self, value: crate::datadogV2::model::RelationshipToRoles) -> Self {
        self.roles = Some(value);
        self
    }
}

impl Default for UserResponseRelationships {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for UserResponseRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UserResponseRelationshipsVisitor;
        impl<'a> Visitor<'a> for UserResponseRelationshipsVisitor {
            type Value = UserResponseRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut org: Option<crate::datadogV2::model::RelationshipToOrganization> = None;
                let mut other_orgs: Option<crate::datadogV2::model::RelationshipToOrganizations> =
                    None;
                let mut other_users: Option<crate::datadogV2::model::RelationshipToUsers> = None;
                let mut roles: Option<crate::datadogV2::model::RelationshipToRoles> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "org" => {
                            if v.is_null() {
                                continue;
                            }
                            org = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "other_orgs" => {
                            if v.is_null() {
                                continue;
                            }
                            other_orgs = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "other_users" => {
                            if v.is_null() {
                                continue;
                            }
                            other_users =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "roles" => {
                            if v.is_null() {
                                continue;
                            }
                            roles = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = UserResponseRelationships {
                    org,
                    other_orgs,
                    other_users,
                    roles,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UserResponseRelationshipsVisitor)
    }
}

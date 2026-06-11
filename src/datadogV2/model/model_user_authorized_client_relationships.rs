// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Relationships for a user authorized client.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UserAuthorizedClientRelationships {
    /// Relationship to the OAuth2 client that was authorized.
    #[serde(rename = "oauth2_client")]
    pub oauth2_client: crate::datadogV2::model::UserAuthorizedClientRelationshipOAuth2Client,
    /// Relationship to the scopes granted to the OAuth2 client.
    #[serde(rename = "scopes")]
    pub scopes: crate::datadogV2::model::UserAuthorizedClientRelationshipScopes,
    /// Relationship to the user who granted this authorization.
    #[serde(rename = "user")]
    pub user: crate::datadogV2::model::UserAuthorizedClientRelationshipUser,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UserAuthorizedClientRelationships {
    pub fn new(
        oauth2_client: crate::datadogV2::model::UserAuthorizedClientRelationshipOAuth2Client,
        scopes: crate::datadogV2::model::UserAuthorizedClientRelationshipScopes,
        user: crate::datadogV2::model::UserAuthorizedClientRelationshipUser,
    ) -> UserAuthorizedClientRelationships {
        UserAuthorizedClientRelationships {
            oauth2_client,
            scopes,
            user,
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

impl<'de> Deserialize<'de> for UserAuthorizedClientRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UserAuthorizedClientRelationshipsVisitor;
        impl<'a> Visitor<'a> for UserAuthorizedClientRelationshipsVisitor {
            type Value = UserAuthorizedClientRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut oauth2_client: Option<
                    crate::datadogV2::model::UserAuthorizedClientRelationshipOAuth2Client,
                > = None;
                let mut scopes: Option<
                    crate::datadogV2::model::UserAuthorizedClientRelationshipScopes,
                > = None;
                let mut user: Option<
                    crate::datadogV2::model::UserAuthorizedClientRelationshipUser,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "oauth2_client" => {
                            oauth2_client =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "scopes" => {
                            scopes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "user" => {
                            user = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let oauth2_client =
                    oauth2_client.ok_or_else(|| M::Error::missing_field("oauth2_client"))?;
                let scopes = scopes.ok_or_else(|| M::Error::missing_field("scopes"))?;
                let user = user.ok_or_else(|| M::Error::missing_field("user"))?;

                let content = UserAuthorizedClientRelationships {
                    oauth2_client,
                    scopes,
                    user,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UserAuthorizedClientRelationshipsVisitor)
    }
}

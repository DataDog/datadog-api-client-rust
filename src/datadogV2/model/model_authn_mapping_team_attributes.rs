// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Team attributes.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AuthNMappingTeamAttributes {
    /// Unicode representation of the avatar for the team, limited to a single grapheme
    #[serde(rename = "avatar", default, with = "::serde_with::rust::double_option")]
    pub avatar: Option<Option<String>>,
    /// Banner selection for the team
    #[serde(rename = "banner", default, with = "::serde_with::rust::double_option")]
    pub banner: Option<Option<i64>>,
    /// The team's identifier
    #[serde(rename = "handle")]
    pub handle: Option<String>,
    /// The number of links belonging to the team
    #[serde(rename = "link_count")]
    pub link_count: Option<i32>,
    /// The name of the team
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// A brief summary of the team, derived from the `description`
    #[serde(
        rename = "summary",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub summary: Option<Option<String>>,
    /// The number of users belonging to the team
    #[serde(rename = "user_count")]
    pub user_count: Option<i32>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AuthNMappingTeamAttributes {
    pub fn new() -> AuthNMappingTeamAttributes {
        AuthNMappingTeamAttributes {
            avatar: None,
            banner: None,
            handle: None,
            link_count: None,
            name: None,
            summary: None,
            user_count: None,
            _unparsed: false,
        }
    }

    pub fn avatar(mut self, value: Option<String>) -> Self {
        self.avatar = Some(value);
        self
    }

    pub fn banner(mut self, value: Option<i64>) -> Self {
        self.banner = Some(value);
        self
    }

    pub fn handle(mut self, value: String) -> Self {
        self.handle = Some(value);
        self
    }

    pub fn link_count(mut self, value: i32) -> Self {
        self.link_count = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn summary(mut self, value: Option<String>) -> Self {
        self.summary = Some(value);
        self
    }

    pub fn user_count(mut self, value: i32) -> Self {
        self.user_count = Some(value);
        self
    }
}

impl Default for AuthNMappingTeamAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AuthNMappingTeamAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AuthNMappingTeamAttributesVisitor;
        impl<'a> Visitor<'a> for AuthNMappingTeamAttributesVisitor {
            type Value = AuthNMappingTeamAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut avatar: Option<Option<String>> = None;
                let mut banner: Option<Option<i64>> = None;
                let mut handle: Option<String> = None;
                let mut link_count: Option<i32> = None;
                let mut name: Option<String> = None;
                let mut summary: Option<Option<String>> = None;
                let mut user_count: Option<i32> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "avatar" => {
                            avatar = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "banner" => {
                            banner = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "handle" => {
                            if v.is_null() {
                                continue;
                            }
                            handle = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "link_count" => {
                            if v.is_null() {
                                continue;
                            }
                            link_count = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "summary" => {
                            summary = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "user_count" => {
                            if v.is_null() {
                                continue;
                            }
                            user_count = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = AuthNMappingTeamAttributes {
                    avatar,
                    banner,
                    handle,
                    link_count,
                    name,
                    summary,
                    user_count,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AuthNMappingTeamAttributesVisitor)
    }
}

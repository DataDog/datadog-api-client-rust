// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Team attributes in hierarchy link context
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TeamHierarchyLinkTeamAttributes {
    /// The team's avatar
    #[serde(rename = "avatar", default, with = "::serde_with::rust::double_option")]
    pub avatar: Option<Option<String>>,
    /// The team's banner
    #[serde(rename = "banner")]
    pub banner: Option<i64>,
    /// The team's handle
    #[serde(rename = "handle")]
    pub handle: String,
    /// Whether the team is managed
    #[serde(rename = "is_managed")]
    pub is_managed: Option<bool>,
    /// Whether the team has open membership
    #[serde(rename = "is_open_membership")]
    pub is_open_membership: Option<bool>,
    /// The number of links for the team
    #[serde(rename = "link_count")]
    pub link_count: Option<i64>,
    /// The team's name
    #[serde(rename = "name")]
    pub name: String,
    /// The team's summary
    #[serde(
        rename = "summary",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub summary: Option<Option<String>>,
    /// The number of users in the team
    #[serde(rename = "user_count")]
    pub user_count: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TeamHierarchyLinkTeamAttributes {
    pub fn new(handle: String, name: String) -> TeamHierarchyLinkTeamAttributes {
        TeamHierarchyLinkTeamAttributes {
            avatar: None,
            banner: None,
            handle,
            is_managed: None,
            is_open_membership: None,
            link_count: None,
            name,
            summary: None,
            user_count: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn avatar(mut self, value: Option<String>) -> Self {
        self.avatar = Some(value);
        self
    }

    pub fn banner(mut self, value: i64) -> Self {
        self.banner = Some(value);
        self
    }

    pub fn is_managed(mut self, value: bool) -> Self {
        self.is_managed = Some(value);
        self
    }

    pub fn is_open_membership(mut self, value: bool) -> Self {
        self.is_open_membership = Some(value);
        self
    }

    pub fn link_count(mut self, value: i64) -> Self {
        self.link_count = Some(value);
        self
    }

    pub fn summary(mut self, value: Option<String>) -> Self {
        self.summary = Some(value);
        self
    }

    pub fn user_count(mut self, value: i64) -> Self {
        self.user_count = Some(value);
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

impl<'de> Deserialize<'de> for TeamHierarchyLinkTeamAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TeamHierarchyLinkTeamAttributesVisitor;
        impl<'a> Visitor<'a> for TeamHierarchyLinkTeamAttributesVisitor {
            type Value = TeamHierarchyLinkTeamAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut avatar: Option<Option<String>> = None;
                let mut banner: Option<i64> = None;
                let mut handle: Option<String> = None;
                let mut is_managed: Option<bool> = None;
                let mut is_open_membership: Option<bool> = None;
                let mut link_count: Option<i64> = None;
                let mut name: Option<String> = None;
                let mut summary: Option<Option<String>> = None;
                let mut user_count: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "avatar" => {
                            avatar = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "banner" => {
                            if v.is_null() {
                                continue;
                            }
                            banner = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "handle" => {
                            handle = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_managed" => {
                            if v.is_null() {
                                continue;
                            }
                            is_managed = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_open_membership" => {
                            if v.is_null() {
                                continue;
                            }
                            is_open_membership =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "link_count" => {
                            if v.is_null() {
                                continue;
                            }
                            link_count = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
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
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let handle = handle.ok_or_else(|| M::Error::missing_field("handle"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = TeamHierarchyLinkTeamAttributes {
                    avatar,
                    banner,
                    handle,
                    is_managed,
                    is_open_membership,
                    link_count,
                    name,
                    summary,
                    user_count,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TeamHierarchyLinkTeamAttributesVisitor)
    }
}

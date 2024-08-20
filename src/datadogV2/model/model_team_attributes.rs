// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Team attributes
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TeamAttributes {
    /// Unicode representation of the avatar for the team, limited to a single grapheme
    #[serde(rename = "avatar", default, with = "::serde_with::rust::double_option")]
    pub avatar: Option<Option<String>>,
    /// Banner selection for the team
    #[serde(rename = "banner", default, with = "::serde_with::rust::double_option")]
    pub banner: Option<Option<i64>>,
    /// Creation date of the team
    #[serde(rename = "created_at")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Free-form markdown description/content for the team's homepage
    #[serde(
        rename = "description",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub description: Option<Option<String>>,
    /// The team's identifier
    #[serde(rename = "handle")]
    pub handle: String,
    /// Collection of hidden modules for the team
    #[serde(rename = "hidden_modules")]
    pub hidden_modules: Option<Vec<String>>,
    /// The number of links belonging to the team
    #[serde(rename = "link_count")]
    pub link_count: Option<i32>,
    /// Modification date of the team
    #[serde(rename = "modified_at")]
    pub modified_at: Option<chrono::DateTime<chrono::Utc>>,
    /// The name of the team
    #[serde(rename = "name")]
    pub name: String,
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
    /// Collection of visible modules for the team
    #[serde(rename = "visible_modules")]
    pub visible_modules: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TeamAttributes {
    pub fn new(handle: String, name: String) -> TeamAttributes {
        TeamAttributes {
            avatar: None,
            banner: None,
            created_at: None,
            description: None,
            handle,
            hidden_modules: None,
            link_count: None,
            modified_at: None,
            name,
            summary: None,
            user_count: None,
            visible_modules: None,
            additional_properties: std::collections::BTreeMap::new(),
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

    pub fn created_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn description(mut self, value: Option<String>) -> Self {
        self.description = Some(value);
        self
    }

    pub fn hidden_modules(mut self, value: Vec<String>) -> Self {
        self.hidden_modules = Some(value);
        self
    }

    pub fn link_count(mut self, value: i32) -> Self {
        self.link_count = Some(value);
        self
    }

    pub fn modified_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.modified_at = Some(value);
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

    pub fn visible_modules(mut self, value: Vec<String>) -> Self {
        self.visible_modules = Some(value);
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

impl<'de> Deserialize<'de> for TeamAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TeamAttributesVisitor;
        impl<'a> Visitor<'a> for TeamAttributesVisitor {
            type Value = TeamAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut avatar: Option<Option<String>> = None;
                let mut banner: Option<Option<i64>> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut description: Option<Option<String>> = None;
                let mut handle: Option<String> = None;
                let mut hidden_modules: Option<Vec<String>> = None;
                let mut link_count: Option<i32> = None;
                let mut modified_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut name: Option<String> = None;
                let mut summary: Option<Option<String>> = None;
                let mut user_count: Option<i32> = None;
                let mut visible_modules: Option<Vec<String>> = None;
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
                            banner = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "handle" => {
                            handle = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "hidden_modules" => {
                            if v.is_null() {
                                continue;
                            }
                            hidden_modules =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "link_count" => {
                            if v.is_null() {
                                continue;
                            }
                            link_count = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_at" => {
                            if v.is_null() {
                                continue;
                            }
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "visible_modules" => {
                            if v.is_null() {
                                continue;
                            }
                            visible_modules =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = TeamAttributes {
                    avatar,
                    banner,
                    created_at,
                    description,
                    handle,
                    hidden_modules,
                    link_count,
                    modified_at,
                    name,
                    summary,
                    user_count,
                    visible_modules,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TeamAttributesVisitor)
    }
}

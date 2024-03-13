// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Team update attributes
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TeamUpdateAttributes {
    /// Unicode representation of the avatar for the team, limited to a single grapheme
    #[serde(rename = "avatar", default, with = "::serde_with::rust::double_option")]
    pub avatar: Option<Option<String>>,
    /// Banner selection for the team
    #[serde(rename = "banner", default, with = "::serde_with::rust::double_option")]
    pub banner: Option<Option<i64>>,
    /// An identifier for the color representing the team
    #[serde(rename = "color")]
    pub color: Option<i32>,
    /// Free-form markdown description/content for the team's homepage
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// The team's identifier
    #[serde(rename = "handle")]
    pub handle: String,
    /// Collection of hidden modules for the team
    #[serde(rename = "hidden_modules")]
    pub hidden_modules: Option<Vec<String>>,
    /// The name of the team
    #[serde(rename = "name")]
    pub name: String,
    /// Collection of visible modules for the team
    #[serde(rename = "visible_modules")]
    pub visible_modules: Option<Vec<String>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TeamUpdateAttributes {
    pub fn new(handle: String, name: String) -> TeamUpdateAttributes {
        TeamUpdateAttributes {
            avatar: None,
            banner: None,
            color: None,
            description: None,
            handle,
            hidden_modules: None,
            name,
            visible_modules: None,
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

    pub fn color(mut self, value: i32) -> Self {
        self.color = Some(value);
        self
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn hidden_modules(mut self, value: Vec<String>) -> Self {
        self.hidden_modules = Some(value);
        self
    }

    pub fn visible_modules(mut self, value: Vec<String>) -> Self {
        self.visible_modules = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for TeamUpdateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TeamUpdateAttributesVisitor;
        impl<'a> Visitor<'a> for TeamUpdateAttributesVisitor {
            type Value = TeamUpdateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut avatar: Option<Option<String>> = None;
                let mut banner: Option<Option<i64>> = None;
                let mut color: Option<i32> = None;
                let mut description: Option<String> = None;
                let mut handle: Option<String> = None;
                let mut hidden_modules: Option<Vec<String>> = None;
                let mut name: Option<String> = None;
                let mut visible_modules: Option<Vec<String>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "avatar" => {
                            avatar = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "banner" => {
                            banner = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "color" => {
                            if v.is_null() {
                                continue;
                            }
                            color = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
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
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "visible_modules" => {
                            if v.is_null() {
                                continue;
                            }
                            visible_modules =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let handle = handle.ok_or_else(|| M::Error::missing_field("handle"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = TeamUpdateAttributes {
                    avatar,
                    banner,
                    color,
                    description,
                    handle,
                    hidden_modules,
                    name,
                    visible_modules,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TeamUpdateAttributesVisitor)
    }
}
